use std::{env::consts::OS, fs::File, path::Path, process::Command};

fn main() {
    println!("=== Launcher ===");
    let repo_link = option_env!("REPO_LINK").unwrap_or("unknown");
    println!("Repo link: {}\n", repo_link);
    let current_version = env!("CARGO_PKG_VERSION");
    println!("Current version: {}.", current_version);
    let version_get = reqwest::blocking::get(format!(
        "{}/releases/latest/download/version.txt",
        repo_link
    ));
    let cli = Path::new(if OS == "windows" {
        ".\\csengeto_cli.exe"
    } else {
        "./csengeto_cli"
    });
    if version_get.is_ok() {
        let ver = version_get.unwrap().text().unwrap();
        println!("Upstream version: {}.", ver);
        if (current_version.to_string() != ver.replace(" ", "")) || !cli.exists() {
            println!("New update available, downloading it...");
            let mut launcher_get = reqwest::blocking::get(format!(
                "{}/releases/latest/download/launcher{}",
                repo_link,
                if OS == "windows" { ".exe" } else { "" }
            ))
            .unwrap();
            let mut main_get = reqwest::blocking::get(format!(
                "{}/releases/latest/download/csengeto_cli{}",
                repo_link,
                if OS == "windows" { ".exe" } else { "" }
            ))
            .unwrap();

            let mut cli = File::create(if OS == "windows" {
                ".\\csengeto_cli.exe"
            } else {
                "./csengeto_cli"
            })
            .unwrap();

            let mut launcher = File::create(if OS == "windows" {
                ".\\launcher_new.exe"
            } else {
                "./launcher_new"
            })
            .unwrap();
            main_get.copy_to(&mut cli).unwrap();
            launcher_get.copy_to(&mut launcher).unwrap();
            println!("Download done, starting new...")
        }
    }
    println!("=== CLI ===");
    Command::new(if OS == "windows" {
        ".\\csengeto_cli.exe"
    } else {
        "./csengeto_cli"
    })
    .spawn()
    .unwrap();
    return;
}
