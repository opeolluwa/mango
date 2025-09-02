use std::path::Path;

use piper_rs::synth::PiperSpeechSynthesizer;
use serde::{Deserialize, Serialize};

use crate::{error::AudifyError, extractor::extract_text, languages::Languages};

#[derive(Debug, Serialize, Deserialize)]
pub struct Audify {
    /// language or model to use see https://github.com/rhasspy/piper/blob/master/VOICES.md
    language: Languages,
    /// where to put the generated audio file
    export_path: String,
    /// the text passed to the library
    source: String,
    /// path to <model_name>.onnx.json
    config_path: String,
    sid: i64,
}

impl Default for Audify {
    fn default() -> Self {
        Self {
            language: Default::default(),
            config_path: "".to_string(),
            export_path: ".".to_string(),
            source: Default::default(),
            sid: 90i64,
        }
    }
}

impl Audify {
    /// Create a new Audify instance
    ///
    /// ### Example
    /// ```
    /// use libaudify::{core::Audify, error::AudifyError};
    ///
    /// /**
    ///  * Just download the models before running:
    ///  * cargo run --example audio
    ///  */
    /// fn main() -> Result<(), AudifyError> {
    ///     let config_path = "en_US-libritts_r-medium.onnx.json";
    ///     let source_text = "hey man does this work?";
    ///     let pdf_path = "test.pdf";
    ///     let raw_export_path = "out_raw.wav";
    ///     let pdf_export_path = "out_pdf.wav";
    ///
    ///     let audify_rs = Audify::new(config_path);
    ///
    ///     audify_rs.synthesize_text(source_text, raw_export_path)?;
    ///     audify_rs.synthesize_pdf(pdf_path, pdf_export_path)?;
    ///
    ///     println!("done");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(config_path: &str) -> Self {
        Self {
            config_path: config_path.to_string(),
            ..Default::default()
        }
    }

    /// generate audio from raw text
    pub fn synthesize_text(&self, source_text: &str, export_path: &str) -> Result<(), AudifyError> {
        let text = source_text.to_string();

        let model = piper_rs::from_config_path(Path::new(&self.config_path))
            .map_err(AudifyError::ModelLoadError)?;

        model.set_speaker(self.sid);

        let synth = PiperSpeechSynthesizer::new(model)
            .map_err(AudifyError::PiperSpeechSynthesizerInitializationError)?;
        synth
            .synthesize_to_file(Path::new(export_path), text, None)
            .map_err(AudifyError::FileSynthesizeError)?;
        Ok(())
    }

    /// generate audio from PDF
    pub fn synthesize_pdf(&self, pdf_path: &str, export_path: &str) -> Result<(), AudifyError> {
        let text = extract_text(pdf_path)?;

        let model = piper_rs::from_config_path(Path::new(&self.config_path))
            .map_err(AudifyError::ModelLoadError)?;

        model.set_speaker(self.sid);

        let synth = PiperSpeechSynthesizer::new(model)
            .map_err(AudifyError::PiperSpeechSynthesizerInitializationError)?;
        synth
            .synthesize_to_file(Path::new(export_path), text, None)
            .map_err(AudifyError::FileSynthesizeError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audify_new() {
        let audify = Audify::new("en_US-libritts_r-medium.onnx.json");
        assert_eq!(audify.config_path, "en_US-libritts_r-medium.onnx.json");
    }

    #[test]
    fn test_audify_default() {
        let audify = Audify::default();
        assert_eq!(audify.language, Languages::default());
        assert_eq!(audify.export_path, ".");
        assert_eq!(audify.source, "");
        assert_eq!(audify.sid, 80);
    }

    #[test]
    fn test_audify_synthesize_text() {
        let audify = Audify::new("../../../resources/models/en_US-libritts_r-medium.onnx.json");
        let result = audify.synthesize_text("Hello, world!", "test.wav");
        assert!(result.is_ok());
        // Clean up test file
        // std::fs::remove_file("test.wav").unwrap_or(());
    }
}
