use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::WavToMp3ConverterError;

pub struct WavToMp3Converter {
    command: Command,
}

impl WavToMp3Converter {
    pub fn new() -> Self {
        WavToMp3Converter {
            command: Command::new("lame"),
        }
    }

    /// Convert a WAV file to MP3 and return the output path
    pub fn convert_file(&mut self, wav_path: &str) -> Result<PathBuf, WavToMp3ConverterError> {
        let wav_path = Path::new(wav_path);
        if wav_path.extension().and_then(|ext| ext.to_str()) != Some("wav") {
            return Err(WavToMp3ConverterError::InavlidFileFormat);
        }

        self.command.arg(wav_path);

        let mp3_path: PathBuf = wav_path.with_extension("mp3");

        self.command
            .output()
            .map_err(|_| WavToMp3ConverterError::LameError)?;

        Ok(mp3_path)
    }
}
