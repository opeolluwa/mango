#[cfg(test)]
#[test]
fn test_convert_to_mp3() {
    let mut converter = crate::WavToMp3Converter::new();

    let input_path = "test.wav";
    let output_path = "test.mp3";

    let outfile = converter.convert_file(input_path).unwrap();
    assert_eq!(outfile.to_str().unwrap(), output_path)
}
