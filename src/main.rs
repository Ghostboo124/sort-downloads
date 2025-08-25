use std::{ffi::OsStr, fs, io, path::{Path, PathBuf}};

fn read_dir(path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(entries)
}

fn recursive_read_dir(path: &Path) -> Option<Vec<PathBuf>> {
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

fn check_dir(path: &Path) -> bool {
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

fn main() -> io::Result<()> {
    if let Ok(entries) = read_dir(Path::new(r"C:\Users\perkinsal\Downloads")){
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
            OsStr::new("ppt"),
            OsStr::new("pptx"),
            OsStr::new("xlsx"),
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
        ];
        let image_exts = [
            OsStr::new("svg"),
            OsStr::new("png"),
            OsStr::new("jpg"),
            OsStr::new("jpeg"),
            OsStr::new("webp"),
        ];
        let installer_exts = [
            OsStr::new("msi"),
            OsStr::new("msix"),
        ];
        let pdf_exts = [
            OsStr::new("pdf"),
        ];

        for folder in ignored {
            check_dir(Path::new(&format!(r"C:\Users\perkinsal\Downloads\{}", folder)));
        }

        // The order in which `read_dir` returns entries is not guaranteed. If reproducible
        // ordering is required the entries should be explicitly sorted.

        for entry in entries {
            if let Some(name) = entry.file_name()
                                    .and_then(|s| s.to_str()) {
                if ignored.contains(&name) {
                    continue;
                }
                match entry.extension() {
                    Some(ext) => {
                        if archive_exts.contains(&ext) {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!(r"C:\Users\perkinsal\Downloads\archives\{}", name)));
                            if rename_result.is_err() {
                                panic!("Rename failed: {}", rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        } else if document_exts.contains(&ext) {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!(r"C:\Users\perkinsal\Downloads\documents\{}", name)));
                            if rename_result.is_err() {
                                panic!("Rename failed: {}", rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        } else if executable_exts.contains(&ext) {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!(r"C:\Users\perkinsal\Downloads\executables\{}", name)));
                            if rename_result.is_err() {
                                panic!("Rename failed: {}", rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        } else if image_exts.contains(&ext) {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!(r"C:\Users\perkinsal\Downloads\images\{}", name)));
                            if rename_result.is_err() {
                                panic!("Rename failed: {}", rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        } else if installer_exts.contains(&ext) {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!(r"C:\Users\perkinsal\Downloads\installers\{}", name)));
                            if rename_result.is_err() {
                                panic!("Rename failed: {}", rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        } else if pdf_exts.contains(&ext) {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!(r"C:\Users\perkinsal\Downloads\pdf\{}", name)));
                            if rename_result.is_err() {
                                panic!("Rename failed: {}", rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        } else {
                            let rename_result = fs::rename(entry.as_path(), Path::new(&format!(r"C:\Users\perkinsal\Downloads\other\{}", name)));
                            if rename_result.is_err() {
                                panic!("Rename failed: {}", rename_result.err().expect("No error when there should have been an error?").to_string());
                            }
                        }
                    }
                    None => {
                        let rename_result = fs::rename(entry.as_path(), Path::new(&format!(r"C:\Users\perkinsal\Downloads\other\{}", name)));
                        if rename_result.is_err() {
                            panic!("Rename failed: {}", rename_result.err().expect("No error when there should have been an error?").to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}