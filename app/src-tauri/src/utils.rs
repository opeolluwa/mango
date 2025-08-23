pub fn initialize_app_directories(
    app_data_dir: &std::path::Path,
    config: &crate::config::AppConfig,
) -> std::io::Result<()> {
    std::fs::create_dir_all(app_data_dir.join(config.media_dir_name))?;
    Ok(())
}
