use std::sync::Arc;
use crate::app::services::sftp::SftpServerService;
use crate::settings::Settings;

mod core;
mod app;
mod settings;

use std::error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    println!("create services");

    let settings: Arc<Settings> = Arc::new(Settings::new().map_err(|err| err.to_string())?);
    let _ = SftpServerService::new_sftp_server_service(&settings)?;

    println!("good buy");
    Ok(())
}
