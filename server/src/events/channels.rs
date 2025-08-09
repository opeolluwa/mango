use std::fmt::Display;

#[derive(Debug)]
pub enum EventChannel {
    DocumentConvertedToAudio,
    ConvertDocumentToAudio,
    Default,
}

impl Display for EventChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let channel = match self {
            EventChannel::ConvertDocumentToAudio => "convert-document-to-audio",
            EventChannel::DocumentConvertedToAudio => "document-converted-to-audio",
            EventChannel::Default => "default",
        };

        write!(f, "aers-{channel}-channel")
    }
}

impl From<String> for EventChannel {
    fn from(channel_string: String) -> Self {
        // Remove the "aers-" prefix and "-channel" suffix to get the base channel name
        let clean_channel = channel_string
            .strip_prefix("aers-")
            .and_then(|s| s.strip_suffix("-channel"))
            .unwrap_or(&channel_string);

        match clean_channel {
            "document-converted" => EventChannel::DocumentConvertedToAudio,
            "convert-document" => EventChannel::ConvertDocumentToAudio,
            _ => EventChannel::Default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats_expected_strings() {
        let convert = EventChannel::ConvertDocumentToAudio;
        assert_eq!(
            convert.to_string(),
            "aers-convert-document-to-audio-channel"
        );

        let converted = EventChannel::DocumentConvertedToAudio;
        assert_eq!(
            converted.to_string(),
            "aers-document-converted-to-audio-channel"
        );

        let default = EventChannel::Default;
        assert_eq!(default.to_string(), "aers-default-channel");
    }

    #[test]
    fn from_string_parses_prefixed_and_suffixed_variants() {
        let ch: EventChannel = "aers-convert-document-channel".to_string().into();
        assert!(matches!(ch, EventChannel::ConvertDocumentToAudio));

        let ch: EventChannel = "aers-document-converted-channel".to_string().into();
        assert!(matches!(ch, EventChannel::DocumentConvertedToAudio));
    }

    #[test]
    fn from_string_defaults_on_unknown_values() {
        let ch: EventChannel = "".to_string().into();
        assert!(matches!(ch, EventChannel::Default));

        let ch: EventChannel = "random-text-here".to_string().into();
        assert!(matches!(ch, EventChannel::Default));

        // Note: current implementation expects base names without "-to-audio"
        // so a fully formatted display string falls back to Default
        let ch: EventChannel = "aers-convert-document-to-audio-channel".to_string().into();
        assert!(matches!(ch, EventChannel::Default));

        let ch: EventChannel = "aers-email".to_string().into();
        assert!(matches!(ch, EventChannel::Default));
    }
}
