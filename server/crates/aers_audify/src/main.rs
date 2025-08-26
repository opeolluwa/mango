use aers_audify::Audify;

fn main() {
    let config_path = "./resources/models/en_US-libritts_r-medium.onnx.json";
    let source_text = "god bless you so much, bye bye";
    let raw_export_path = "./luke-6-.wav";

    let audify_rs = Audify::new(config_path);

    audify_rs
        .synthesize_text(source_text, raw_export_path)
        .unwrap();

    println!("done");
}
