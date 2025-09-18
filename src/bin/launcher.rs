fn main() {
    let repo_link = option_env!("REPO_LINK").unwrap_or("unknown");
    println!("Repo link: {}", repo_link);
    // ...rest of your launcher logic...
}
