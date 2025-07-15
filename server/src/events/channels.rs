use std::fmt::Display;

pub enum RedisMessageChannel {
    Email,
    FileUpload,
    FileConverted,
    Mp3Converted,
}

impl Display for RedisMessageChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let channel = match self {
            RedisMessageChannel::Email => "aers-email-channel",
            RedisMessageChannel::FileUpload => "aers-file-upload",
            RedisMessageChannel::FileConverted => "aers-file-converted",
            RedisMessageChannel::Mp3Converted => "aers-mp3-converted",
        };

        write!(f, "{}", channel.to_string())
    }
}
