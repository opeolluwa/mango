// #[cfg(test)]
// #[test]
// fn test_convert_to_mp3() {
//     let converter = crate::WavToMp3Converter::new();

//     let input_path = "test.wav";
//     let output_path = "test.mp3";

//     std::fs::File::create(input_path).expect("Failed to create input file");
  

//     // Perform the conversion
//     let result = converter.convert(input_path, output_path);

//     println!("Conversion result: {:?}", result);
//     // Check if the conversion was successful
//     assert!(result.is_ok());

//     // Clean up test files
//     // std::fs::remove_file(input_path).unwrap();
//     // std::fs::remove_file(output_path).unwrap();
// }
