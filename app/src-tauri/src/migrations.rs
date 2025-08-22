use tauri_plugin_sql::{Migration, MigrationKind};
/// Load database migrations from embedded SQL files
pub fn load_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            description: "create_default_tables",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812181420_create_default_tables.sql"),
        },
        Migration {
            version: 2,
            description: "add_time_stamps_to_playlist_table",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812181622_add_timestanp_to_playlist.sql"),
        },
        Migration {
            version: 3,
            description: "change_is_loved_to_boolean",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812181730_change_is_loved_to_boolean.sql"),
        },
        Migration {
            version: 4,
            description: "create_app_settings_table",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812181908_create_app_settings_table.sql"),
        },
        Migration {
            version: 5,
            description: "create_app_personalization_table",
            kind: MigrationKind::Up,
            sql: include_str!(
                "../migrations/20250812181958_create_app_personalization_table_table.sql"
            ),
        },
        Migration {
            version: 6,
            description: "create_cached_user_table",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812182051_create_cached_user_table.sql"),
        },
        Migration {
            version: 7,
            description: "remove_user_details_from_app_personalization",
            kind: MigrationKind::Up,
            sql: include_str!(
                "../migrations/20250812182303_remove_user_from_app_personalization.sql"
            ),
        },
        Migration {
            version: 8,
            description: "create_app_settings_table_if_not_exists",
            kind: MigrationKind::Up,
            sql: include_str!("../migrations/20250812182430_create_app_settings_if_not_exist.sql"),
        },
    ]
}
