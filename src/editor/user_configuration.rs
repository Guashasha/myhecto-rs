use std::{fs, io::Error};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserControls {
    pub move_left: char,
    pub move_right: char,
    pub move_up: char,
    pub move_down: char,
    pub insert_mode: char,
}

impl Default for UserControls {
    fn default() -> Self {
        Self {
            move_left: 'h',
            move_right: 'l',
            move_up: 'k',
            move_down: 'j',
            insert_mode: 'i',
        }
    }
}

pub(crate) fn get_user_controls() -> Result<UserControls, Error> {
    let controls: UserControls;

    match read_config_file() {
        Ok(file) => controls = serde_json::from_str(&file)?,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => return Ok(UserControls::default()),
            _ => return Err(err),
        },
    }

    Ok(controls)
}

fn read_config_file() -> Result<String, Error> {
    let config_file = "config.json";
    let contents = fs::read_to_string(config_file)?;
    Ok(contents)
}
