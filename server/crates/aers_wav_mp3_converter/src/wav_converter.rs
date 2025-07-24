use std::{
    path::{Path, PathBuf},
    process::Command,
};

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
    pub fn convert_and_export(&mut self, wav_path: &str) -> Result<String, String> {
        let wav_path = Path::new(wav_path);

        // Ensure the file has a `.wav` extension
        if wav_path.extension().and_then(|ext| ext.to_str()) != Some("wav") {
            return Err("Input file is not a .wav file".into());
        }

        // Generate the .mp3 path by replacing the extension
        let mp3_path: PathBuf = wav_path.with_extension("mp3");

        // Perform the conversion
        self.convert(
            wav_path.to_str().ok_or("Invalid WAV path")?,
            mp3_path.to_str().ok_or("Failed to create MP3 path")?,
        )?;

        // Return the final path as a string
        Ok(mp3_path.to_string_lossy().to_string())
    }

    /// Low-level convert method using the `lame` binary
    pub fn convert(&mut self, input_path: &str, output_path: &str) -> Result<(), String> {
        let output = self
            .command
            .arg(input_path)
            .arg(output_path)
            .output()
            .map_err(|e| format!("Failed to run `lame`: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "LAME error: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }
}
