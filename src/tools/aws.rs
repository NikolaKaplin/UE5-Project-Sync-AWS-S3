use anyhow::{Context, Result};
use aws_sdk_s3::{
    config::{Credentials, Region},
    Client,
};
use indicatif::{ProgressBar, ProgressStyle};
use std::{env, path::{Path}, time::{Instant}};
use aws_sdk_s3::types::CompletedMultipartUpload;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use dotenv::dotenv;

pub async fn upload_to_yandex_s3(
    file_path: &Path,
    bucket_name: &str,
    object_key: &str,
) -> Result<()> {
    dotenv().ok();
    println!("Starting upload to Yandex S3...");

    let access_key = env::var("YANDEX_ACCESS_KEY_ID")?;
    let secret_key = env::var("YANDEX_SECRET_ACCESS_KEY_ID")?;
    let endpoint = env::var("AWS_ENDPOINT")?;
    let region = env::var("AWS_REGION")?;

    let credentials = Credentials::new(access_key, secret_key, None, None, "custom-provider");
    let config = aws_sdk_s3::Config::builder()
        .credentials_provider(credentials)
        .region(Region::new(region))
        .endpoint_url(endpoint)
        .build();

    let client = Client::from_conf(config);

    // Проверяем и очищаем незавершенные загрузки
    cleanup_incomplete_uploads(&client, bucket_name, object_key).await?;

    let file_size = tokio::fs::metadata(file_path).await?.len();
    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} {msg} [{wide_bar}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
            .progress_chars("=> "),
    );
    pb.set_message("Uploading to Yandex S3...");

    // Определяем минимальный размер для составной загрузки (5MB для Yandex Object Storage)
    const MULTIPART_THRESHOLD: u64 = 5 * 1024 * 1024; // 5MB

    if file_size < MULTIPART_THRESHOLD {
        // Простая загрузка для маленьких файлов
        pb.set_message("Uploading (single part)...");

        let body = aws_sdk_s3::primitives::ByteStream::from_path(file_path).await?;

        client
            .put_object()
            .bucket(bucket_name)
            .key(object_key)
            .body(body)
            .send()
            .await?;

        pb.finish_with_message("Upload complete!");
    } else {
        // Составная загрузка для больших файлов
        pb.set_message("Uploading (multipart)...");

        let mut file = File::open(file_path).await?;
        let upload_manager = client
            .create_multipart_upload()
            .bucket(bucket_name)
            .key(object_key)
            .send()
            .await?;

        let upload_id = upload_manager.upload_id().context("No upload ID returned")?.to_string();
        let mut part_number = 1;
        let mut completed_parts = Vec::new();
        let mut buffer = vec![0; 8 * 1024 * 1024]; // 8MB chunks
        let start_time = Instant::now();

        let upload_result = async {
            loop {
                let bytes_read = file.read(&mut buffer).await?;
                if bytes_read == 0 {
                    break;
                }

                let part_data = bytes::Bytes::copy_from_slice(&buffer[..bytes_read]);
                let part_result = client
                    .upload_part()
                    .bucket(bucket_name)
                    .key(object_key)
                    .upload_id(&upload_id)
                    .part_number(part_number)
                    .body(part_data.into())
                    .send()
                    .await?;

                completed_parts.push(
                    aws_sdk_s3::types::CompletedPart::builder()
                        .part_number(part_number)
                        .e_tag(part_result.e_tag.unwrap_or_default())
                        .build(),
                );

                pb.inc(bytes_read as u64);
                part_number += 1;
            }

            Ok::<_, anyhow::Error>(completed_parts)
        }.await;

        match upload_result {
            Ok(completed_parts) => {
                client
                    .complete_multipart_upload()
                    .bucket(bucket_name)
                    .key(object_key)
                    .upload_id(upload_id)
                    .multipart_upload(
                        CompletedMultipartUpload::builder()
                            .set_parts(Some(completed_parts))
                            .build(),
                    )
                    .send()
                    .await?;

                let duration = start_time.elapsed();
                let speed = file_size as f64 / duration.as_secs_f64() / 1024.0 / 1024.0;
                pb.finish_with_message(format!(
                    "Upload complete! Speed: {:.2} MB/s",
                    speed
                ));
            }
            Err(e) => {
                eprintln!("Upload failed, aborting multipart upload: {:?}", e);
                client
                    .abort_multipart_upload()
                    .bucket(bucket_name)
                    .key(object_key)
                    .upload_id(upload_id)
                    .send()
                    .await
                    .ok(); // Игнорируем ошибку отмены
                return Err(e);
            }
        }
    }

    println!("File successfully uploaded to Yandex S3");
    Ok(())
}

pub async fn download_from_yandex_s3(
    file_path: &Path,
    bucket_name: &str,
    object_key: &str,
) -> Result<()> {
    dotenv().ok();
    println!("Starting download from Yandex S3...");


    let access_key = env::var("YANDEX_ACCESS_KEY_ID")?;
    let secret_key = env::var("YANDEX_SECRET_ACCESS_KEY_ID")?;
    let endpoint = env::var("AWS_ENDPOINT")?;
    let region = env::var("AWS_REGION")?;

    let credentials = Credentials::new(access_key, secret_key, None, None, "custom-provider");
    let config = aws_sdk_s3::Config::builder()
        .credentials_provider(credentials)
        .region(Region::new(region))
        .endpoint_url(endpoint)
        .build();

    let client = Client::from_conf(config);

    let head_object = client
        .head_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await?;
    let file_size = head_object.content_length.unwrap_or(0) as u64;

    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} {msg} [{wide_bar}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
            .progress_chars("=> "),
    );
    pb.set_message("Downloading from Yandex S3...");

    let mut file = File::create(file_path).await?;
    let mut stream = client
        .get_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await?;

    let start_time = Instant::now();
    while let Some(bytes) = stream.body.try_next().await? {
        file.write_all(&bytes).await?;
        pb.inc(bytes.len() as u64);
    }

    let duration = start_time.elapsed();
    let speed = file_size as f64 / duration.as_secs_f64() / 1024.0 / 1024.0;
    pb.finish_with_message(format!(
        "Download complete! Speed: {:.2} MB/s",
        speed
    ));

    println!("File successfully downloaded from Yandex S3");
    Ok(())
}

async fn cleanup_incomplete_uploads(
    client: &Client,
    bucket_name: &str,
    object_key: &str,
) -> Result<()> {
    let list_uploads = client
        .list_multipart_uploads()
        .bucket(bucket_name)
        .send()
        .await?;

    if let Some(uploads) = list_uploads.uploads {
        for upload in uploads {
            if upload.key.as_deref() == Some(object_key) {
                println!("Found incomplete upload for key: {}, aborting...", object_key);
                client
                    .abort_multipart_upload()
                    .bucket(bucket_name)
                    .key(object_key)
                    .upload_id(upload.upload_id.unwrap_or_default())
                    .send()
                    .await?;
            }
        }
    }
    Ok(())
}
