use async_std::fs::read;
use std::str;

pub async fn in_root_directory(project: &str) -> bool {
    let content = read("Cargo.toml")
        .await
        .expect("Not in the root of a project!");
    let content = str::from_utf8(&content).unwrap();

    content.contains("workspace") && content.contains(&format!("{}-", project))
}
