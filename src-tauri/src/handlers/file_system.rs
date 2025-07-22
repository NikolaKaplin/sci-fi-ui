use serde::{Deserialize, Serialize};
use std::{fs, path::Path, time::SystemTime};
use tauri;

#[derive(Debug, Serialize, Deserialize)]
pub struct FsItem {
    name: String,
    path: String,
    is_dir: bool,
    size: u64,
    modified: u64,
    icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FsResponse {
    path: String,
    items: Vec<FsItem>,
}

#[tauri::command]
pub async fn list_dir(path: String) -> Result<FsResponse, String> {
    let dir_path = Path::new(&path);
    if !dir_path.exists() {
        return Err("Path does not exist".into());
    }
    if !dir_path.is_dir() {
        return Err("Path is not a directory".into());
    }

    let mut items = Vec::new();

    for entry in fs::read_dir(dir_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;

        let modified = metadata
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH)
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        items.push(FsItem {
            name: path.file_name().unwrap().to_string_lossy().into_owned(),
            path: path.to_string_lossy().into_owned(),
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            modified,
            icon: None,
        });
    }

    items.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    Ok(FsResponse {
        path: dir_path.to_string_lossy().into_owned(),
        items,
    })
}

#[tauri::command]
pub fn get_drives() -> Vec<FsItem> {
    #[cfg(target_os = "windows")]
    {
        use std::path::Path;

        let mut drives = Vec::new();
        for drive_letter in b'A'..=b'Z' {
            let drive = format!("{}:\\", drive_letter as char);
            if Path::new(&drive).exists() {
                drives.push(FsItem {
                    name: format!("Drive {}:", drive_letter as char),
                    path: drive.clone(),
                    is_dir: true,
                    size: 0,
                    modified: 0,
                    icon: Some("drive".to_string()),
                });
            }
        }
        drives
    }

    #[cfg(not(target_os = "windows"))]
    {
        vec![FsItem {
            name: "Root".to_string(),
            path: "/".to_string(),
            is_dir: true,
            size: 0,
            modified: 0,
        }]
    }
}

#[tauri::command]
pub fn get_parent_dir(path: String) -> String {
    let path = std::path::Path::new(&path);
    if let Some(parent) = path.parent() {
        parent.to_string_lossy().to_string()
    } else {
        "/".to_string()
    }
}
