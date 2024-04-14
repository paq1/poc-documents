use std::net::{SocketAddr, TcpStream};

use ssh2::Session;

use crate::core::services::FileService;
use crate::settings::Settings;

pub struct SftpServerService {
    session: Session
}


impl SftpServerService {
    pub fn new_sftp_server_service(settings: &Settings) -> Result<Self, String> {
        let host = settings.sftp.host;
        let port = settings.sftp.port;

        let tcp: TcpStream = TcpStream::connect(SocketAddr::from((host, port)))
            .map(|tcp| {
                println!("[{host:?}:{port:?}] -> connection tcp ok !");
                tcp
            })
            .map_err(|err| {
                let custom_err = format!("[{host:?}:{port:?}] -> connection tcp failed !");
                let tcp_err = err.to_string();
                format!("{custom_err}\n{tcp_err}")
            })?;

        let mut session = Session::new().map_err(|err| err.to_string())?;
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();

        println!("start authentication");
        if session.authenticated() {
            println!("session ssh ouverte");
            Ok(
                Self {
                    session
                }
            )
        } else {
            let err = "erreur d'authentification";
            Err(err.to_string())
        }
    }
}

impl FileService for SftpServerService {
    fn upload_file(&self) -> bool {
        false
    }
}