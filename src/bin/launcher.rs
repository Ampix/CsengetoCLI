use std::{
    env::consts::OS,
    process::{Child, Command},
};

#[tokio::main]
async fn main() -> Child {
    let repo_link = option_env!("REPO_LINK").unwrap_or("unknown");
    println!("Repo link: {}", repo_link);
    let version_get = reqwest::get(format!(
        "{}/releases/latest/download/version.txt",
        repo_link
    ))
    .await;
    if version_get.is_ok() {
        let current_version = env!("CARGO_PKG_VERSION");
        println!("Current version: {}", current_version);
        let ver = version_get.unwrap().text().await.unwrap();
        if current_version.to_string() != ver {
        } else {
            println!("No update found, starting current...")
        }
    } else {
        println!("Version check failed, starting current version...");
    }
    return Command::new(if OS == "windows" {
        "csengeto_cli.exe"
    } else {
        "./csengeto_cli"
    })
    .spawn()
    .unwrap();
}
