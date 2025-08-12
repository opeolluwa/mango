use std::{
    path::{Path, PathBuf},
    process::Command,
};

use aers_wav_mp3_converter::WavToMp3ConverterError;

fn main() -> Result<(), WavToMp3ConverterError> {
    let wav_path = Path::new("./out_raw.wav");
    if wav_path.extension().and_then(|ext| ext.to_str()) != Some("wav") {
        return Err(WavToMp3ConverterError::InavlidFileFormat);
    }

    let mut lame_command = Command::new("lame");
    lame_command.arg(wav_path);

    let mp3_path: PathBuf = wav_path.with_extension("mp3");

    println!("{:#?}", mp3_path);

    Ok(())
}
