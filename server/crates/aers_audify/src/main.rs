use aers_audify::{Audify, AudifyError};

fn main() {
    let config_path = "./resources/models/en_US-libritts_r-medium.onnx.json";
    let source_text = "hey man does this work?";
    let raw_export_path = "./test.wav";

    let audify_rs = Audify::new(config_path);

    audify_rs
        .synthesize_text(source_text, raw_export_path)
        .unwrap();

    println!("done");
}
