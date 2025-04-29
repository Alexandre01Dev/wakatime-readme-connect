use reqwest::blocking::{Client};
use serde_json::json;
use base64::{engine::general_purpose, Engine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "ghp_..."; // Ton token GitHub
    let repo = "alexandre-le-majestueux/mon-repo";
    let path = "README.md";
    let branch = "main";

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

    let sha = resp["sha"].as_str().unwrap();
    let content_base64 = resp["content"].as_str().unwrap().replace("\n", "");
    let decoded = general_purpose::STANDARD.decode(&content_base64)?;
    let mut content = String::from_utf8(decoded)?;

    // Modif du fichier
    content.push_str("\nAjout par Alexandre le Majestueux");

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

    println!("✅ Fichier mis à jour !");
    Ok(())
}
