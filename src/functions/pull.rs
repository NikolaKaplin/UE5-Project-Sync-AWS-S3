use std::{env, fs};
use std::path::{Path, PathBuf};
use anyhow::Context;
use dotenv::dotenv;
use crate::tools;

pub async fn restore_backup(project_name: &str, target_path: &Path) -> anyhow::Result<()> {
    dotenv().ok();
    println!("Restoring project '{}' to: {}", project_name, target_path.display());

    let temp_dir = PathBuf::from("backup_temp");
    if !temp_dir.exists() {
        fs::create_dir(&temp_dir).context("Failed to create temp directory")?;
    }

    let download_path = temp_dir.join(format!("UE5_Restore_{}.7z", project_name));
    let bucket_name = env::var("BUCKET_NAME").context("BUCKET_NAME not set")?;
    let object_key = format!("backups/{}.7z", project_name);

    tools::aws::download_from_yandex_s3(&download_path, &bucket_name, &object_key).await?;
    tools::compressing::extract_7z_archive(&download_path, target_path).await?;
    fs::remove_file(&download_path).context("Failed to remove temporary download file")?;

    Ok(())
}