use random_word::Lang;

pub fn generate_file_name() -> String {
    random_word::all_len(4, Lang::En).unwrap().join("_")
}
