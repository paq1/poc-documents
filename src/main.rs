use std::sync::Arc;
use crate::app::services::sftp::SftpServerService;
use crate::settings::Settings;

mod core;
mod app;
mod settings;

fn main() -> Result<(), String> {
    dotenv::dotenv().ok();

    println!("create services");

    let settings: Arc<Settings> = Arc::new(Settings::new().map_err(|err| err.to_string())?);
    let _ = SftpServerService::new_sftp_server_service(&settings).unwrap();

    println!("good buy");
    Ok(())
}
