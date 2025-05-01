use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

#[derive(Deserialize, Serialize)]
pub struct Config {
    wakatime_url: String,
    wakatime_api_token: String,
    wakatime_user: String,
    wakatime_platform: Platform,
    github_api_token: String,
    github_user: String,
    github_repo: String,
    github_branch: String,
    github_md_file: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub enum Platform {
    Wakatime,
    Wakapi,
}
impl Config {
    pub fn get_wakatime_url(&self) -> &str {
        &self.wakatime_url
    }

    pub fn get_wakatime_api_token(&self) -> &str {
        &self.wakatime_api_token
    }

    pub fn get_wakatime_user(&self) -> &str {
        &self.wakatime_user
    }

    pub fn get_wakatime_platform(&self) -> &Platform {
        &self.wakatime_platform
    }

    pub fn get_github_api_token(&self) -> &str {
        &self.github_api_token
    }

    pub fn get_github_user(&self) -> &str {
        &self.github_user
    }

    pub fn get_github_repo(&self) -> &str {
        &self.github_repo
    }

    pub fn get_github_branch(&self) -> &str {
        &self.github_branch
    }

    pub fn get_github_md_file(&self) -> &str {
        &self.github_md_file
    }
}

pub fn load_config() -> Config {
    match fs::read_to_string("config.yaml") {
        Ok(config_content) => serde_yaml::from_str(&config_content)
            .expect("Impossible to parse the configuration file"),
        Err(_) => {
            let default_config = Config {
                wakatime_url: "https://wakatime.com/api/v1".to_string(),
                wakatime_api_token: "your_api_token".to_string(),
                wakatime_user: "current".to_string(),
                wakatime_platform: Platform::Wakatime,
                github_api_token: "your_github_api_token".to_string(),
                github_user: "your_github_username".to_string(),
                github_repo: "your_github_repo".to_string(),
                github_branch: "main".to_string(),
                github_md_file: "README.md".to_string(),
            };

            let default_content = serde_yaml::to_string(&default_config)
                .expect("Impossible to serialize the default configuration");
            let mut file = fs::File::create("config.yaml")
                .expect("Impossible to create the configuration file");
            file.write_all(default_content.as_bytes())
                .expect("Impossible to write to the configuration file");

            println!(
                "File config.yaml not found. A default configuration file has been created. Please edit it with your Wakatime API token and username."
            );
            default_config
        }
    }
}
