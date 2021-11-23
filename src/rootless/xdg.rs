use std::error::Error;

pub fn xdg_runtime_dir() -> Result<String, Box<dyn Error>> {
    let mut runtime_dir = std::env::var("XDG_RUNTIME_DIR")?;
    if !runtime_dir.is_empty() {
        return Ok(runtime_dir);
    }

    let euid = std::env::var("ROOTLESSKIT_PARENT_EUID")?;
    if !euid.is_empty() {
        runtime_dir = format!("/run/user/{}", euid);
        return Ok(runtime_dir);
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "environment variable XDG_RUNTIME_DIR is not set, see https://rootlesscontaine.rs/getting-started/common/login/",
    )))
}

pub fn xdg_config_home() -> Result<String, Box<dyn Error>> {
    let xch = std::env::var("XDG_CONFIG_HOME")?;
    if !xch.is_empty() {
        return Ok(xch);
    }
    let home = std::env::var("HOME")?;
    if home.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "environment variable HOME is not set",
        )));
    }
    Ok(format!("{}/.config", home))
}

pub fn xdg_data_home() -> Result<String, Box<dyn Error>> {
    let xdh = std::env::var("XDG_DATA_HOME")?;
    if !xdh.is_empty() {
        return Ok(xdh);
    }
    let home = std::env::var("HOME")?;
    if home.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "environment variable HOME is not set",
        )));
    }
    Ok(format!("{}/.local/share", home))
}
