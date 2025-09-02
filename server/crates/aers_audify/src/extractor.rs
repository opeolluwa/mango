use extractous::Extractor;

use crate::error::AudifyError;

#[allow(dead_code)]
#[deprecated = "breaking API due to missing awt package, use extract text"]
pub(super) fn extract_pdf_source(pdf_path: &str) -> Result<String, AudifyError> {
    let extractor = Extractor::new();
    let (content, _metadata) = extractor
        .extract_file_to_string(pdf_path)
        .map_err(|_| AudifyError::AudioEndoingError)?;

    Ok(content)
}

pub(super) fn extract_text(pdf_path: &str) -> Result<String, AudifyError> {
    let content = pdf_extract::extract_text(pdf_path)
        .map_err(|err| AudifyError::SynthesisError(err.to_string()))?;

    Ok(content)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_pdf_source() {
        let pdf_path = "./sample.pdf";
        let content = extract_text(pdf_path).ok();

        assert!(content.is_some())
    }
}
