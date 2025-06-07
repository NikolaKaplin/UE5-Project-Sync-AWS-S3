use std::path::Path;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use sevenz_rust::SevenZWriter;
use walkdir::WalkDir;

pub async fn compress_project_to_7z(project_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    println!("Starting compression...");

    let exclude_dirs = ["DerivedDataCache", "Intermediate", "Binaries", ".git"];
    let exclude_extensions = [".pdb", ".bak", ".tmp"];

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner} {msg}")?,
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Counting files...");

    let total_files = WalkDir::new(project_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            !exclude_dirs.iter().any(|dir| path.to_string_lossy().contains(dir))
                && !exclude_extensions.iter().any(|ext| path.to_string_lossy().ends_with(ext))
                && path.is_file()
        })
        .count();

    pb.finish_and_clear();

    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} {msg} [{wide_bar}] {pos}/{len} ({eta})")?
            .progress_chars("=> "),
    );
    pb.set_message("Compressing...");

    let mut writer = SevenZWriter::create(output_path)?;

    for entry in WalkDir::new(project_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let relative_path = path.strip_prefix(project_path)?;

        if exclude_dirs.iter().any(|dir| path.to_string_lossy().contains(dir))
            || exclude_extensions
            .iter()
            .any(|ext| path.to_string_lossy().ends_with(ext))
        {
            continue;
        }

        if path.is_file() {
            let _path_str = relative_path.to_string_lossy().replace('\\', "/");
            writer.push_source_path(path, |_| true)?;
            pb.inc(1);
        }
    }

    writer.finish()?;
    pb.finish_with_message("Compression complete!");

    println!(
        "Archive created successfully at: {}",
        output_path.display()
    );
    Ok(())
}

pub async fn extract_7z_archive(archive_path: &Path, extract_path: &Path) -> anyhow::Result<()> {
    println!("Starting extraction...");

    if !extract_path.exists() {
        tokio::fs::create_dir_all(extract_path).await?;
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner} {msg}")?,
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Extracting files...");

    let file = std::fs::File::open(archive_path)?;
    sevenz_rust::decompress(file, extract_path)?;

    pb.finish_with_message("Extraction complete!");
    println!("Archive successfully extracted to: {}", extract_path.display());
    Ok(())
}
