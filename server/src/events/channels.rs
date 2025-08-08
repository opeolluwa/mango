use std::fmt::Display;

#[derive(Debug, Default)]
pub enum RedisMessageChannel {
    Email,
    FileUploaded,
    FileConverted,
    Mp3Converted,
    #[default]
    Default,
    DocumentConverted,
    ConvertDocument,
}

impl Display for RedisMessageChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let channel = match self {
            RedisMessageChannel::Email => "email",
            RedisMessageChannel::FileUploaded => "file-uploaded",
            RedisMessageChannel::FileConverted => "file-converted",
            RedisMessageChannel::Mp3Converted => "mp3-converted",
            RedisMessageChannel::Default => "default",
            RedisMessageChannel::ConvertDocument => "convert-document",
            RedisMessageChannel::DocumentConverted => "document-converted",
        };

        write!(f, "aers-{}-channel", channel)
    }
}

impl From<String> for RedisMessageChannel {
    fn from(channel_string: String) -> Self {
        // Remove the "aers-" prefix and "-channel" suffix to get the base channel name
        let clean_channel = channel_string
            .strip_prefix("aers-")
            .and_then(|s| s.strip_suffix("-channel"))
            .unwrap_or(&channel_string);

        match clean_channel {
            "email" => RedisMessageChannel::Email,
            "file-uploaded" => RedisMessageChannel::FileUploaded,
            "file-converted" => RedisMessageChannel::FileConverted,
            "mp3-converted" => RedisMessageChannel::Mp3Converted,
            "document-converted" => RedisMessageChannel::DocumentConverted,
            "convert-document" => RedisMessageChannel::ConvertDocument,
            _ => RedisMessageChannel::Default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_email() {
        let channel_string = "aers-email-channel".to_string();
        let channel: RedisMessageChannel = channel_string.into();
        assert!(matches!(channel, RedisMessageChannel::Email));
    }

    #[test]
    fn test_from_string_file_uploaded() {
        let channel_string = "aers-file-uploaded-channel".to_string();
        let channel: RedisMessageChannel = channel_string.into();
        assert!(matches!(channel, RedisMessageChannel::FileUploaded));
    }

    #[test]
    fn test_from_string_file_converted() {
        let channel_string = "aers-file-converted-channel".to_string();
        let channel: RedisMessageChannel = channel_string.into();
        assert!(matches!(channel, RedisMessageChannel::FileConverted));
    }

    #[test]
    fn test_from_string_mp3_converted() {
        let channel_string = "aers-mp3-converted-channel".to_string();
        let channel: RedisMessageChannel = channel_string.into();
        assert!(matches!(channel, RedisMessageChannel::Mp3Converted));
    }

    #[test]
    fn test_from_string_unknown_channel() {
        let channel_string = "aers-unknown-channel".to_string();
        let channel: RedisMessageChannel = channel_string.into();
        assert!(matches!(channel, RedisMessageChannel::Default));
    }

    #[test]
    fn test_from_string_malformed_prefix() {
        let channel_string = "email-channel".to_string();
        let channel: RedisMessageChannel = channel_string.into();
        assert!(matches!(channel, RedisMessageChannel::Default));
    }

    #[test]
    fn test_from_string_malformed_suffix() {
        let channel_string = "aers-email".to_string();
        let channel: RedisMessageChannel = channel_string.into();
        assert!(matches!(channel, RedisMessageChannel::Default));
    }

    #[test]
    fn test_from_string_empty() {
        let channel_string = "".to_string();
        let channel: RedisMessageChannel = channel_string.into();
        assert!(matches!(channel, RedisMessageChannel::Default));
    }

    #[test]
    fn test_from_string_random_text() {
        let channel_string = "random-text-here".to_string();
        let channel: RedisMessageChannel = channel_string.into();
        assert!(matches!(channel, RedisMessageChannel::Default));
    }

    #[test]
    fn test_roundtrip_display_and_from() {
        let original_channels = vec![
            RedisMessageChannel::Email,
            RedisMessageChannel::FileUploaded,
            RedisMessageChannel::FileConverted,
            RedisMessageChannel::Mp3Converted,
        ];

        for original in original_channels {
            let string_repr = original.to_string();
            let converted: RedisMessageChannel = string_repr.into();
            assert_eq!(
                std::mem::discriminant(&original),
                std::mem::discriminant(&converted)
            );
        }
    }

    #[test]
    fn test_display_format() {
        let email_channel = RedisMessageChannel::Email;
        assert_eq!(email_channel.to_string(), "aers-email-channel");

        let file_uploaded_channel = RedisMessageChannel::FileUploaded;
        assert_eq!(
            file_uploaded_channel.to_string(),
            "aers-file-uploaded-channel"
        );

        let file_converted_channel = RedisMessageChannel::FileConverted;
        assert_eq!(
            file_converted_channel.to_string(),
            "aers-file-converted-channel"
        );

        let mp3_converted_channel = RedisMessageChannel::Mp3Converted;
        assert_eq!(
            mp3_converted_channel.to_string(),
            "aers-mp3-converted-channel"
        );

        let default_channel = RedisMessageChannel::Default;
        assert_eq!(default_channel.to_string(), "aers-default-channel");
    }
}
