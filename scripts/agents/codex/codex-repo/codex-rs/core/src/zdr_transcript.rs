use crate::models::ResponseItem;

/// Transcript that needs to be maintained for ZDR clients for which
/// previous_response_id is not available, so we must include the transcript
/// with every API call. This must include each `function_call` and its
/// corresponding `function_call_output`.
#[derive(Debug, Clone)]
pub(crate) struct ZdrTranscript {
    /// The oldest items are at the beginning of the vector.
    items: Vec<ResponseItem>,
}

impl ZdrTranscript {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Returns a clone of the contents in the transcript.
    pub(crate) fn contents(&self) -> Vec<ResponseItem> {
        self.items.clone()
    }

    /// `items` is ordered from oldest to newest.
    pub(crate) fn record_items<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = ResponseItem>,
    {
        for item in items {
            if is_api_message(&item) {
                // Note agent-loop.ts also does filtering on some of the fields.
                self.items.push(item.clone());
            }
        }
    }
}

/// Anything that is not a system message or "reasoning" message is considered
/// an API message.
fn is_api_message(message: &ResponseItem) -> bool {
    match message {
        ResponseItem::Message { role, .. } => role.as_str() != "system",
        ResponseItem::FunctionCall { .. } => true,
        ResponseItem::FunctionCallOutput { .. } => true,
        _ => false,
    }
}
