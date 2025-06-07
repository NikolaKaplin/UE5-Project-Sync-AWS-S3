use std::{env, fs};
use std::path::PathBuf;
use anyhow::Context;
use crate::tools;
use dotenv::dotenv;
pub async fn create_backup() -> anyhow::Result<()> {
    dotenv().ok();
    let current_dir = env::current_dir().context("Failed to get current directory")?;
    println!("Current project directory: {}", current_dir.display());

    let temp_dir = PathBuf::from("backup_temp");
    if !temp_dir.exists() {
        fs::create_dir(&temp_dir).context("Failed to create temp directory")?;
    }

    let output_7z_path = temp_dir.join("UE5_Project_Backup.7z");
    let bucket_name = env::var("BUCKET_NAME").context("BUCKET_NAME not set")?;
    let object_key = "ue5_project_backup.7z";

    tools::compressing::compress_project_to_7z(&current_dir, &output_7z_path).await?;
    tools::aws::upload_to_yandex_s3(&output_7z_path, &bucket_name, object_key).await?;
    fs::remove_file(&output_7z_path).context("Failed to remove temporary backup file")?;

    Ok(())
}