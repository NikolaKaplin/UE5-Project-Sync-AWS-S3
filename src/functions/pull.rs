use std::{env, fs};
use std::path::PathBuf;
use anyhow::Context;
use crate::tools::aws::download_from_yandex_s3;
use crate::tools::compressing::extract_7z_archive;
use dotenv::dotenv;
pub async fn restore_backup() -> anyhow::Result<()> {
    dotenv().ok();
    println!("Enter path to restore the project (leave empty to use current directory):");
    let mut restore_path = String::new();
    std::io::stdin().read_line(&mut restore_path)?;
    let restore_path = restore_path.trim();

    let extract_path = if restore_path.is_empty() {
        env::current_dir().context("Failed to get current directory")?
    } else {
        PathBuf::from(restore_path)
    };

    println!("Will restore to: {}", extract_path.display());

    let temp_dir = PathBuf::from("backup_temp");
    if !temp_dir.exists() {
        fs::create_dir(&temp_dir).context("Failed to create temp directory")?;
    }

    let download_path = temp_dir.join("UE5_Project_Restore.7z");
    let bucket_name = env::var("BUCKET_NAME").context("BUCKET_NAME not set")?;
    let object_key = "ue5_project_backup.7z";

    download_from_yandex_s3(&download_path, &bucket_name, object_key).await?;
    extract_7z_archive(&download_path, &extract_path).await?;
    fs::remove_file(&download_path).context("Failed to remove temporary download file")?;

    println!("Restore completed successfully!");
    Ok(())
}