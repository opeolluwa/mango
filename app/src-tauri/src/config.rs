pub struct AppConfig {
    pub model_config_file: &'static str,
    pub media_dir_name: &'static str,
    pub database_file: &'static str,
    pub database_path: &'static str,
    pub lame_sidecar: &'static str,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model_config_file: "resources/en_US-libritts_r-medium.onnx.json",
            media_dir_name: "audify",
            database_file: "sqlite:eckko.db",
            database_path: "eckko.db",
            lame_sidecar: "lame",
        }
    }
}