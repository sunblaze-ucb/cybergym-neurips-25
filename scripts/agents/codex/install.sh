#!/bin/bash -eux
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

CODEX_REPO="$SCRIPT_DIR/codex-repo"

# clone repo
if [ ! -d "$CODEX_REPO" ]; then
    git clone -b cybergym git@github.com:sunblaze-ucb/cybergym-codex.git $CODEX_REPO
fi

# build the codex image 
trap "popd >> /dev/null" EXIT
CODEX_CLI_DIR="$CODEX_REPO/codex-cli"
pushd "$CODEX_CLI_DIR" >> /dev/null || {
  echo "Error: Failed to change directory to $CODEX_CLI_DIR"
  exit 1
}
pnpm install
pnpm run build
rm -rf ./dist/openai-codex-*.tgz
pnpm pack --pack-destination ./dist
mv ./dist/openai-codex-*.tgz ./dist/codex.tgz
docker build -t cybergym/codex -f "./Dockerfile.cybergym" .