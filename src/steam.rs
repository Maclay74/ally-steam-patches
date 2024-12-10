use dirs::data_local_dir;
use glob::glob;
use std::fs;
use std::path::PathBuf;
use sysinfo;

pub struct Steam;

impl Steam {
    pub fn is_running(&self) -> bool {
        let mut sys = sysinfo::System::new_all();

        sys.refresh_all();

        sys.processes().values().any(|p| p.name() == "steam")
    }

    pub fn get_ui_folder(&self) -> Option<PathBuf> {
        let share_dir = data_local_dir()?;
        let steam_dir = share_dir.join("Steam/steamui");

        if !steam_dir.is_dir() {
            return None;
        }

        Some(steam_dir)
    }

    pub fn get_chunk_file(&self) -> Option<PathBuf> {
        let folder = self.get_ui_folder()?;
        let pattern = folder.join("chunk*");
        let threshold = 1_048_576;
        let entries = glob(&pattern.to_str()?).unwrap();

        for entry in entries {
            let path = entry.unwrap();
            if fs::metadata(&path).unwrap().len() > threshold {
                return Some(path);
            }
        }
        None
    }
}
