use thiserror::Error;

#[derive(Debug, Error)]
pub enum WavToMp3ConverterError {
    #[error("Input file is not a .wav file")]
    InavlidFileFormat,
    #[error("error extracting")]
    LameError,
}
