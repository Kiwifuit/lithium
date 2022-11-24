use crate::err::PathError;
use glob::glob;
use std::env::var;
use std::path::PathBuf;

fn get_home() -> PathBuf {
    let Ok(local_app_dir) = var("LOCALAPPDATA") else {
        panic!("LOCALAPPDATA is not set, {}", if cfg!(target_os = "windows") {
            "are you sure this system is properly configured?"
        } else {
            "are you sure you're running on Windows?"
        });
    };

    PathBuf::from(local_app_dir).join("Google/Chrome/User Data")
}

pub fn get_all_profiles() -> Result<Vec<PathBuf>, PathError> {
    let mut home = get_home().to_str().unwrap().to_string();
    let mut res = vec![];

    home.extend(['/', 'P', 'r', 'o', 'f', 'i', 'l', 'e', ' ', '*'].iter());

    let globber = match glob(home.as_str()) {
        Ok(g) => g,
        Err(e) => return Err(PathError::GlobError(e.to_string())),
    };

    for profile in globber {
        match profile {
            Ok(p) => res.push(p),
            Err(e) => return Err(PathError::SearchError(e.to_string())),
        };
    }

    Ok(res)
}

pub fn get_local_state() -> Result<PathBuf, PathError> {
    let local_state = get_home().join("Local State");

    if !local_state.exists() {
        return Err(PathError::LocalStateDoesNotExist(local_state));
    }

    Ok(local_state)
}
