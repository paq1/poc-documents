use std::net::{SocketAddr, TcpStream};

use ssh2::Session;

use crate::core::custom_error::CustomError;
use crate::core::services::FileService;
use crate::settings::Settings;

pub struct SftpServerService {
    session: Session
}

impl SftpServerService {
    pub fn new_sftp_server_service(settings: &Settings) -> Result<Self, CustomError> {

        let tcp: TcpStream = SftpServerService::create_tcp_stream(settings)?;

        let mut session = Session::new()
            .map_err(|err| CustomError::new(err.to_string().as_str()))?;
        session.set_tcp_stream(tcp);
        session.handshake().map_err(|err| CustomError::new(err.to_string().as_str()))?;
        session.userauth_password(settings.sftp.user.as_str(), settings.sftp.pswd.as_str())
            .map_err(|err| CustomError::new(err.to_string().as_str()))?;

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
            Err(CustomError::new(err))
        }
    }

    fn create_tcp_stream(settings: &Settings) -> Result<TcpStream, CustomError> {
        let host = settings.sftp.host;
        let port = settings.sftp.port;

        TcpStream::connect(SocketAddr::from((host, port)))
            .map(|tcp| {
                println!("[{host:?}:{port:?}] -> connection tcp ok !");
                tcp
            })
            .map_err(|err| { CustomError::new("test") })
    }
}

impl FileService for SftpServerService {
    fn upload_file(&self) -> bool {
        false
    }
}