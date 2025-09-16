use std::{env::home_dir, ffi::OsStr, fs, io, path::{Path, PathBuf, MAIN_SEPARATOR_STR}, result};

pub fn read_dir(path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(entries)
}

pub fn recursive_read_dir(path: &Path) -> Option<Vec<PathBuf>> {
    let mut entries: Vec<PathBuf> = vec![];
    let tld: Vec<PathBuf> = read_dir(path).ok()?;
    for entry in tld {
        if entry.is_dir() {
            entries.append(&mut recursive_read_dir(entry.as_path())?);
            continue;
        }
        entries.push(entry);
    }

    Some(entries)
}

pub fn check_dir(path: &Path) -> bool {
    let result: Result<bool, io::Error> = fs::exists(path);
    match result {
        Ok(c) => {
            if !c {
                let _ = fs::create_dir(path);
                return true
            } else {
                return c
            }
        }
        Err(c) => {
            panic!("Failed to check if dir exists: {}", c)
        }
    }
}

fn main() -> result::Result<(), io::Error> {
    let base_path: &String = &format!(r"{}{}Downloads", home_dir().unwrap().to_str().unwrap(), MAIN_SEPARATOR_STR);
    let archive_path: &String = &format!(r"{}{}archives", base_path, MAIN_SEPARATOR_STR);
    let executable_path: &String = &format!(r"{}{}executables", base_path, MAIN_SEPARATOR_STR);
    let document_path: &String = &format!(r"{}{}documents", base_path, MAIN_SEPARATOR_STR);
    let image_path: &String = &format!(r"{}{}images", base_path, MAIN_SEPARATOR_STR);
    let installer_path: &String = &format!(r"{}{}installers", base_path, MAIN_SEPARATOR_STR);
    let other_path: &String = &format!(r"{}{}other", base_path, MAIN_SEPARATOR_STR);
    let pdf_path: &String = &format!(r"{}{}pdf", base_path, MAIN_SEPARATOR_STR);

    if let Ok(entries) = read_dir(Path::new(base_path)) {
        let ignored = [
            "archives",
            "executables",
            "documents",
            "images",
            "installers",
            "other",
            "pdf",
        ];
        let archive_exts = [
            OsStr::new("zip"),
            OsStr::new("rar"),
            OsStr::new("7z"),
            OsStr::new("xz"),
            OsStr::new("gz"),
            OsStr::new("tar"),
        ];
        let document_exts = [
            OsStr::new("doc"),
            OsStr::new("docx"),
            OsStr::new("docm"),
            OsStr::new("ppt"),
            OsStr::new("pptx"),
            OsStr::new("pptm"),
            OsStr::new("xlsx"),
            OsStr::new("xlsm"),
            OsStr::new("txt"),
            OsStr::new("rtf"),
            OsStr::new("md"),
        ];
        let executable_exts = [
            OsStr::new("exe"),
            OsStr::new("com"),
            OsStr::new("bat"),
            OsStr::new("cmd"),
            OsStr::new("ps1"),
            OsStr::new("sh"),
            OsStr::new("base"),
            OsStr::new("zsh"),
        ];
        let image_exts = [
            OsStr::new("svg"),
            OsStr::new("png"),
            OsStr::new("jpg"),
            OsStr::new("jpeg"),
            OsStr::new("webp"),
            OsStr::new("gif"),
            OsStr::new("dng"),
            OsStr::new("cr2"),
        ];
        let installer_exts = [
            OsStr::new("msi"),
            OsStr::new("msix"),
            OsStr::new("AppImage"),
        ];
        let pdf_exts = [
            OsStr::new("pdf"),
        ];

        for folder in ignored {
            check_dir(Path::new(&format!(r"{}{}{}", base_path, MAIN_SEPARATOR_STR, folder)));
        }

        for entry in entries {
            if let Some(name) = entry.file_name()
                                    .and_then(|s| s.to_str()) {
                if ignored.contains(&name) {
                    continue;
                }
                match entry.extension() {
                    Some(ext) => {
                        if archive_exts.contains(&ext) {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!("{}{}{}", archive_path.as_str(), MAIN_SEPARATOR_STR, name)));
                            if rename_result.is_err() {
                                panic!("Rename failed with file {}: {}", name, rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        } else if document_exts.contains(&ext) {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!("{}{}{}", document_path.as_str(), MAIN_SEPARATOR_STR, name)));
                            if rename_result.is_err() {
                                panic!("Rename failed with file {}: {}", name, rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        } else if executable_exts.contains(&ext) {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!("{}{}{}", executable_path.as_str(), MAIN_SEPARATOR_STR, name)));
                            if rename_result.is_err() {
                                panic!("Rename failed with file {}: {}", name, rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        } else if image_exts.contains(&ext) {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!("{}{}{}", image_path.as_str(), MAIN_SEPARATOR_STR, name)));
                            if rename_result.is_err() {
                                panic!("Rename failed with file {}: {}", name, rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        } else if installer_exts.contains(&ext) {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!("{}{}{}", installer_path.as_str(), MAIN_SEPARATOR_STR, name)));
                            if rename_result.is_err() {
                                panic!("Rename failed with file {}: {}", name, rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        } else if pdf_exts.contains(&ext) {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!("{}{}{}", pdf_path.as_str(), MAIN_SEPARATOR_STR, name)));
                            if rename_result.is_err() {
                                panic!("Rename failed with file {}: {}", name, rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        } else {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!("{}{}{}", other_path.as_str(), MAIN_SEPARATOR_STR, name)));
                            if rename_result.is_err() {
                                panic!("Rename failed with file {}: {}", name, rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        }
                    }
                    None => {
                        let rename_result = fs::rename(entry.as_path(), Path::new(&format!("{}{}{}", other_path.as_str(), MAIN_SEPARATOR_STR, name)));
                        if rename_result.is_err() {
                            panic!("Rename failed with file {}: {}", name, rename_result.err().expect("No error when there should have been an error?").to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
