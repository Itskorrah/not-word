//! Browser preferences share the desktop's tolerant UI codec. No document text is stored here.
use wordcraft_ui_egui::UiState;

pub const KEY: &str = "not-word.ui.v1";
const MAX_BYTES: usize = 2 << 20;

pub fn decode(text: &str) -> Option<UiState> {
    (text.len() <= MAX_BYTES).then(|| UiState::from_json(text.as_bytes())).flatten()
}

pub fn encode(ui: &UiState) -> Result<String, String> {
    let text = serde_json::to_string(ui).map_err(|e| e.to_string())?;
    if text.len() > MAX_BYTES {
        return Err("interface preferences exceed the browser storage budget".into());
    }
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_preferences_keep_valid_fields_and_default_invalid_ones() {
        let ui = decode(r#"{"tab":"Review","highContrast":true,"accent":"violet","workspace":"academic","interfaceSize":125}"#).unwrap();
        let back = decode(&encode(&ui).unwrap()).unwrap();
        assert_eq!(back.tab, "Review");
        assert!(back.high_contrast);
        assert_eq!(back.accent.code(), "violet");
        assert_eq!(back.workspace.code(), "academic");
        let bad = decode(r#"{"tab":"Insert","accent":"unknown","highContrast":"invalid"}"#).unwrap();
        assert_eq!(bad.tab, "Insert");
        assert!(!bad.high_contrast);
        assert_eq!(bad.accent.code(), "teal");
    }

    #[test]
    fn corrupt_or_oversized_browser_preferences_are_rejected() {
        assert!(decode("not json").is_none());
        assert!(decode(&" ".repeat(MAX_BYTES + 1)).is_none());
        assert_eq!(KEY, "not-word.ui.v1");
    }
}
