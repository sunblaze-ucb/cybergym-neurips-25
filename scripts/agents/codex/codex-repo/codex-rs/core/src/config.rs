use crate::approval_mode_cli_arg::parse_sandbox_permission_with_base_path;
use crate::flags::OPENAI_DEFAULT_MODEL;
use crate::protocol::AskForApproval;
use crate::protocol::SandboxPermission;
use crate::protocol::SandboxPolicy;
use dirs::home_dir;
use serde::Deserialize;
use std::path::PathBuf;

/// Embedded fallback instructions that mirror the TypeScript CLI’s default
/// system prompt. These are compiled into the binary so a clean install behaves
/// correctly even if the user has not created `~/.codex/instructions.md`.
const EMBEDDED_INSTRUCTIONS: &str = include_str!("../prompt.md");

/// Application configuration loaded from disk and merged with overrides.
#[derive(Debug, Clone)]
pub struct Config {
    /// Optional override of model selection.
    pub model: String,

    /// Approval policy for executing commands.
    pub approval_policy: AskForApproval,

    pub sandbox_policy: SandboxPolicy,

    /// Disable server-side response storage (sends the full conversation
    /// context with every request). Currently necessary for OpenAI customers
    /// who have opted into Zero Data Retention (ZDR).
    pub disable_response_storage: bool,

    /// System instructions.
    pub instructions: Option<String>,
}

/// Base config deserialized from ~/.codex/config.toml.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct ConfigToml {
    /// Optional override of model selection.
    pub model: Option<String>,

    /// Default approval policy for executing commands.
    pub approval_policy: Option<AskForApproval>,

    // The `default` attribute ensures that the field is treated as `None` when
    // the key is omitted from the TOML. Without it, Serde treats the field as
    // required because we supply a custom deserializer.
    #[serde(default, deserialize_with = "deserialize_sandbox_permissions")]
    pub sandbox_permissions: Option<Vec<SandboxPermission>>,

    /// Disable server-side response storage (sends the full conversation
    /// context with every request). Currently necessary for OpenAI customers
    /// who have opted into Zero Data Retention (ZDR).
    pub disable_response_storage: Option<bool>,

    /// System instructions.
    pub instructions: Option<String>,
}

impl ConfigToml {
    /// Attempt to parse the file at `~/.codex/config.toml`. If it does not
    /// exist, return a default config. Though if it exists and cannot be
    /// parsed, report that to the user and force them to fix it.
    fn load_from_toml() -> std::io::Result<Self> {
        let config_toml_path = codex_dir()?.join("config.toml");
        match std::fs::read_to_string(&config_toml_path) {
            Ok(contents) => toml::from_str::<Self>(&contents).map_err(|e| {
                tracing::error!("Failed to parse config.toml: {e}");
                std::io::Error::new(std::io::ErrorKind::InvalidData, e)
            }),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::info!("config.toml not found, using defaults");
                Ok(Self::default())
            }
            Err(e) => {
                tracing::error!("Failed to read config.toml: {e}");
                Err(e)
            }
        }
    }
}

fn deserialize_sandbox_permissions<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<SandboxPermission>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let permissions: Option<Vec<String>> = Option::deserialize(deserializer)?;

    match permissions {
        Some(raw_permissions) => {
            let base_path = codex_dir().map_err(serde::de::Error::custom)?;

            let converted = raw_permissions
                .into_iter()
                .map(|raw| {
                    parse_sandbox_permission_with_base_path(&raw, base_path.clone())
                        .map_err(serde::de::Error::custom)
                })
                .collect::<Result<Vec<_>, D::Error>>()?;

            Ok(Some(converted))
        }
        None => Ok(None),
    }
}

/// Optional overrides for user configuration (e.g., from CLI flags).
#[derive(Default, Debug, Clone)]
pub struct ConfigOverrides {
    pub model: Option<String>,
    pub approval_policy: Option<AskForApproval>,
    pub sandbox_policy: Option<SandboxPolicy>,
    pub disable_response_storage: Option<bool>,
}

impl Config {
    /// Load configuration, optionally applying overrides (CLI flags). Merges
    /// ~/.codex/config.toml, ~/.codex/instructions.md, embedded defaults, and
    /// any values provided in `overrides` (highest precedence).
    pub fn load_with_overrides(overrides: ConfigOverrides) -> std::io::Result<Self> {
        let cfg: ConfigToml = ConfigToml::load_from_toml()?;
        tracing::warn!("Config parsed from config.toml: {cfg:?}");
        Ok(Self::load_from_base_config_with_overrides(cfg, overrides))
    }

    fn load_from_base_config_with_overrides(cfg: ConfigToml, overrides: ConfigOverrides) -> Self {
        // Instructions: user-provided instructions.md > embedded default.
        let instructions =
            Self::load_instructions().or_else(|| Some(EMBEDDED_INSTRUCTIONS.to_string()));

        // Destructure ConfigOverrides fully to ensure all overrides are applied.
        let ConfigOverrides {
            model,
            approval_policy,
            sandbox_policy,
            disable_response_storage,
        } = overrides;

        let sandbox_policy = match sandbox_policy {
            Some(sandbox_policy) => sandbox_policy,
            None => {
                // Derive a SandboxPolicy from the permissions in the config.
                match cfg.sandbox_permissions {
                    // Note this means the user can explicitly set permissions
                    // to the empty list in the config file, granting it no
                    // permissions whatsoever.
                    Some(permissions) => SandboxPolicy::from(permissions),
                    // Default to read only rather than completely locked down.
                    None => SandboxPolicy::new_read_only_policy(),
                }
            }
        };

        Self {
            model: model.or(cfg.model).unwrap_or_else(default_model),
            approval_policy: approval_policy
                .or(cfg.approval_policy)
                .unwrap_or_else(AskForApproval::default),
            sandbox_policy,
            disable_response_storage: disable_response_storage
                .or(cfg.disable_response_storage)
                .unwrap_or(false),
            instructions,
        }
    }

    fn load_instructions() -> Option<String> {
        let mut p = codex_dir().ok()?;
        p.push("instructions.md");
        std::fs::read_to_string(&p).ok()
    }

    /// Meant to be used exclusively for tests: `load_with_overrides()` should
    /// be used in all other cases.
    pub fn load_default_config_for_test() -> Self {
        Self::load_from_base_config_with_overrides(
            ConfigToml::default(),
            ConfigOverrides::default(),
        )
    }
}

fn default_model() -> String {
    OPENAI_DEFAULT_MODEL.to_string()
}

/// Returns the path to the Codex configuration directory, which is `~/.codex`.
/// Does not verify that the directory exists.
pub fn codex_dir() -> std::io::Result<PathBuf> {
    let mut p = home_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find home directory",
        )
    })?;
    p.push(".codex");
    Ok(p)
}

/// Returns the path to the folder where Codex logs are stored. Does not verify
/// that the directory exists.
pub fn log_dir() -> std::io::Result<PathBuf> {
    let mut p = codex_dir()?;
    p.push("log");
    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that the `sandbox_permissions` field on `ConfigToml` correctly
    /// differentiates between a value that is completely absent in the
    /// provided TOML (i.e. `None`) and one that is explicitly specified as an
    /// empty array (i.e. `Some(vec![])`). This ensures that downstream logic
    /// that treats these two cases differently (default read-only policy vs a
    /// fully locked-down sandbox) continues to function.
    #[test]
    fn test_sandbox_permissions_none_vs_empty_vec() {
        // Case 1: `sandbox_permissions` key is *absent* from the TOML source.
        let toml_source_without_key = "";
        let cfg_without_key: ConfigToml = toml::from_str(toml_source_without_key)
            .expect("TOML deserialization without key should succeed");
        assert!(cfg_without_key.sandbox_permissions.is_none());

        // Case 2: `sandbox_permissions` is present but set to an *empty array*.
        let toml_source_with_empty = "sandbox_permissions = []";
        let cfg_with_empty: ConfigToml = toml::from_str(toml_source_with_empty)
            .expect("TOML deserialization with empty array should succeed");
        assert_eq!(Some(vec![]), cfg_with_empty.sandbox_permissions);

        // Case 3: `sandbox_permissions` contains a non-empty list of valid values.
        let toml_source_with_values = r#"
            sandbox_permissions = ["disk-full-read-access", "network-full-access"]
        "#;
        let cfg_with_values: ConfigToml = toml::from_str(toml_source_with_values)
            .expect("TOML deserialization with valid permissions should succeed");

        assert_eq!(
            Some(vec![
                SandboxPermission::DiskFullReadAccess,
                SandboxPermission::NetworkFullAccess
            ]),
            cfg_with_values.sandbox_permissions
        );
    }

    /// Deserializing a TOML string containing an *invalid* permission should
    /// fail with a helpful error rather than silently defaulting or
    /// succeeding.
    #[test]
    fn test_sandbox_permissions_illegal_value() {
        let toml_bad = r#"sandbox_permissions = ["not-a-real-permission"]"#;

        let err = toml::from_str::<ConfigToml>(toml_bad)
            .expect_err("Deserialization should fail for invalid permission");

        // Make sure the error message contains the invalid value so users have
        // useful feedback.
        let msg = err.to_string();
        assert!(msg.contains("not-a-real-permission"));
    }
}
