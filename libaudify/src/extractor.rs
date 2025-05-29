use crate::error::AudifyError;
use extractous::Extractor;

pub(super) fn extract_pdf_source(pdf_path: &str) -> Result<String, AudifyError> {
    let extractor = Extractor::new();
    // todo: reuse the meta data
    let (content, _metadata) = extractor
        .extract_file_to_string(pdf_path)
        .map_err(|_| AudifyError::AudioEndoingError)?;

    Ok(content)
}
