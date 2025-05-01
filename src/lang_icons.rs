use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

const ICONS_REP: &str =
    "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/%lang%/%lang%-original.svg";
const UNKNOWN_ICON: &str =
    "https://static-00.iconduck.com/assets.00/file-unknown-icon-1775x2048-pyaeuwoe.png";

static LANGUAGE_ICONS: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn search_icon(name: &str) -> String {
    let name_lower = name.to_lowercase();

    {
        let icons = LANGUAGE_ICONS.lock().unwrap();
        if let Some(icon) = icons.get(&name_lower) {
            return icon.clone();
        }
    }

    let icon_rep = ICONS_REP.replace("%lang%", &name_lower);
    let client = reqwest::blocking::Client::new();

    let request_builder = client.get(&icon_rep).header("Accept", "image/*");

    match request_builder.send() {
        Ok(response) => {
            let mut icons = LANGUAGE_ICONS.lock().unwrap();
            if response.status().is_success() {
                icons.insert(name_lower, icon_rep.clone());
                icon_rep
            } else {
                icons.insert(name_lower, UNKNOWN_ICON.to_string());
                UNKNOWN_ICON.to_string()
            }
        }
        Err(e) => {
            let mut icons = LANGUAGE_ICONS.lock().unwrap();
            icons.insert(name_lower, UNKNOWN_ICON.to_string());
            eprintln!("Error: {}", e);
            UNKNOWN_ICON.to_string()
        }
    }
}
