use aers_wav_mp3_converter::WavToMp3Converter;
use std::fs;
use std::path::Path;

fn main() {
    let input = "./aud.wav";

    // Resolve canonical (absolute) path
    let canonical_input = match fs::canonicalize(Path::new(input)) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get canonical path: {}", e);
            return;
        }
    };

    println!("{:#?}", canonical_input.to_str());
    // Convert to MP3
    let result =
        WavToMp3Converter::new().convert_and_export(canonical_input.to_str().unwrap_or(""));

    println!("{:#?}", result);
}
