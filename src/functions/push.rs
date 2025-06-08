use std::{env, fs};
use std::path::{Path, PathBuf};
use anyhow::Context;
use crate::tools;
use dotenv::dotenv;


pub async fn create_backup(project_path: &Path) -> anyhow::Result<()> {
    dotenv().ok();
    println!("Backup project at: {}", project_path.display());

    let temp_dir = PathBuf::from("backup_temp");
    if !temp_dir.exists() {
        fs::create_dir(&temp_dir).context("Failed to create temp directory")?;
    }

    let output_name = format!(
        "UE5_Backup_{}.7z",
        project_path.file_name().unwrap().to_str().unwrap()
    );
    let output_7z_path = temp_dir.join(output_name);

    let bucket_name = env::var("BUCKET_NAME").context("BUCKET_NAME not set")?;
    let object_key = format!(
        "backups/{}.7z",
        project_path.file_name().unwrap().to_str().unwrap()
    );

    tools::compressing::compress_project_to_7z(project_path, &output_7z_path).await?;
    tools::aws::upload_to_yandex_s3(&output_7z_path, &bucket_name, &object_key).await?;
    fs::remove_file(&output_7z_path).context("Failed to remove temporary backup file")?;

    Ok(())
}