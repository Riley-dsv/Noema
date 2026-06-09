use std::{env, path::PathBuf};

pub fn default_database_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let appdata = env::var("APPDATA").expect("APPDATA is not set");

        return PathBuf::from(appdata).join("Noema").join("noema.db");
    }

    #[cfg(target_os = "macos")]
    {
        let home = env::var("HOME").expect("HOME is not set");

        return PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("Noema")
            .join("noema.db");
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Ok(xdg_data_dirs) = env::var("XDG_DATA_DIRS") {
            return PathBuf::from(xdg_data_dirs).join("noema").join("noema.db");
        }

        let home = env::var("HOME").expect("HOME is not set");

        PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("noema")
            .join("noema.db")
    }
}
