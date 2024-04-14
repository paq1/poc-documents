pub mod files;

pub trait FileService {

    fn upload_file(&self) -> bool;

}
