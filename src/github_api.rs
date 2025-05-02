use crate::{config::Config, md_modules};
use base64::{Engine, engine::general_purpose};
use chrono::format;
use lazy_static::lazy_static;
use reqwest::blocking::Client;
use serde_json::json;
use std::sync::Mutex;

// Use lazy_static and Mutex to safely handle static mutable state
lazy_static! {
    static ref LAST_MSG: Mutex<String> = Mutex::new(String::new());
}

pub fn get_and_update(
    config: &Config,
    modules: md_modules::MdDocument,
) -> Result<(), Box<dyn std::error::Error>> {
    let token = config.get_github_api_token();
    let repo = format!("{}/{}", config.get_github_user(), config.get_github_repo());
    let path = config.get_github_md_file();
    let branch = config.get_github_branch();

    let client = Client::new();
    let api_url = format!(
        "https://api.github.com/repos/{}/contents/{}?ref={}",
        repo, path, branch
    );

    // GET du fichier
    let resp: serde_json::Value = client
        .get(&api_url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "rust-client")
        .send()?
        .json()?;

    println!("Response: {}", resp);
    let sha = resp["sha"].as_str().unwrap();
    let content_base64 = resp["content"].as_str().unwrap().replace("\n", "");
    let decoded = general_purpose::STANDARD.decode(&content_base64)?;
    let mut content = String::from_utf8(decoded)?;

    let opt_start = content.find("<!-- START_WAKATIME_BLOCK -->");
    let mut found = false;
    match opt_start {
        Some(start) => {
            let opt_end = content.find("<!-- END_WAKATIME_BLOCK -->");
            match opt_end {
                Some(end) => {
                    found = true;
                    let start = start + "<!-- START_WAKATIME_BLOCK -->".len();
                    content.replace_range(start..end, &format!("\n{}", &modules.render(config)));
                }
                None => {
                    println!("No end block found");
                }
            }
        }
        None => {
            println!("No start block found");
        }
    }

    if !found {
        content.push_str("\n<!-- START_WAKATIME_BLOCK -->\n");
        content.push_str(&modules.render(config));
        content.push_str("\n<!-- END_WAKATIME_BLOCK -->\n");
    }

    // Check if content has changed
    let mut last_msg = LAST_MSG.lock().unwrap();
    if *last_msg == content {
        println!("No changes detected, skipping update.");
        return Ok(());
    }
    *last_msg = content.clone();

    // Encode en base64
    let updated_base64 = general_purpose::STANDARD.encode(&content);

    // PUT du fichier
    let body = json!({
        "message": "Update README.md",
        "content": updated_base64,
        "sha": sha,
        "branch": branch
    });

    let _ = client
        .put(&api_url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "rust-client")
        .json(&body)
        .send()?;

    let date = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("✅ File updated at {}", date);
    Ok(())
}
