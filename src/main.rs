use anyhow::{ Result};
use tokio::{
    io::{ AsyncWriteExt},
};
mod tools;
mod functions;


#[tokio::main]
async fn main() -> Result<()> {
    loop {
        println!("Unreal Engine 5 Backup/Restore Tool");
        println!("write 'push' to create backup (compress and upload to Yandex S3)");
        println!("write 'pull' Restore project (download from Yandex S3 and extract)");
        println!("write exit");
        println!("Enter your command:");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();
        match choice {
            "push" => match functions::push::create_backup().await {
                Ok(_) => println!("Backup completed successfully!"),
                Err(e) => eprintln!("Backup failed: {:?}", e),
            },
            "pull" => match functions::pull::restore_backup().await {
                Ok(_) => println!("Restore completed successfully!"),
                Err(e) => eprintln!("Restore failed: {:?}", e),
            },
            "exit" => break,
            _ => println!("Invalid choice. Please enter 1, 2 or 3."),
        }
    }
    Ok(())
}



