use extractous::Extractor;

use crate::error::AudifyError;

pub(super) fn extract_pdf_source(pdf_path: &str) -> Result<String, AudifyError> {
    let extractor = Extractor::new();
    let (content, _metadata) = extractor
        .extract_file_to_string(pdf_path)
        .map_err(|_| AudifyError::AudioEndoingError)?;

    Ok(content)
}
