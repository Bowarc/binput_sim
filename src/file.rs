pub fn load(path: impl Into<String>) -> Result<String, std::io::Error> {
    let path = path.into();
    let stopwatch = crate::time::Stopwatch::start_new();
    let start_info_message = format!("Loading '{}'", path);

    match std::fs::File::open(path) {
        Ok(mut file) => {
            use std::io::Read as _;

            let mut content = String::new();

            file.read_to_string(&mut content)?;
            debug!("{start_info_message} . . success in {stopwatch}");
            Ok(content)
        }
        Err(e) => {
            // format!("Could not open path: {:?}, {}", path.fs, path.p);
            error!("{} . . error: {e}", start_info_message);
            Err(e)
        }
    }
}

pub fn save(file_name: &str, content: String) -> Result<(), std::io::Error> {
    let base_path = {
        let mut temp = std::env::current_exe().unwrap();
        temp.pop();
        temp.as_os_str()
            .to_str()
            .unwrap()
            .to_string()
            .replace("\\\\?\\", "")
            .replace('\\', "/")
    };

    let path = format!("{base_path}/{file_name}");

    debug!("Saving with path: {path}");

    std::fs::write(path, content)?;
    Ok(())
}
