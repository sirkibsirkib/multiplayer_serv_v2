use std::fs::File;
use std::io::Error;

trait Storable {
    pub fn save_to(path: &Path);
    pub fn load_from(path: &Path) -> Result<Self,Error>;
}

trait Saver {
    fn save<S: Storable>(s: S, path: &Path) {
        unimplemented!()
    }

    fn load<S: Storable>(s: S, path: &Path) -> Result<S,Error> {
        unimplemented!()
    }
}

pub struct ServerSaver {
    folder: &Path,
}

impl ServerSaver {
    pub fn new(folder: &Path) -> Self {
        ServerSaver {
            folder: folder
        }
    }
}

impl Saver for ServerSaver {
    fn open(relative_path: &str) -> Result<File,Error> {
        unimplemented!()
    }
}