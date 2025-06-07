extern crate winres;
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icons/app.ico")
            .set("ProductName", "UE5 Backup Tool")
            .set("FileDescription", "Unreal Engine 5 Backup Utility")
            .set("LegalCopyright", "Copyright © 2023");
        res.compile().unwrap();
    }
}