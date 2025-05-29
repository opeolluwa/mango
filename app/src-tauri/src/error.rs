#[derive(Debug, thiserror::Error)]
pub enum AudioError {
    #[error("Path resolution failed")]
    PathResolutionError,
    #[error("Invalid file name")]
    FileNameError,
    #[error("Audify synthesis error: {0}")]
    SynthesisError(String),
    #[error("Failed to emit event")]
    EmitError(#[from] tauri::Error)
}

impl serde::Serialize for AudioError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
