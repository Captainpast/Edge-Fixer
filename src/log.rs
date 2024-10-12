use std::{fs::File, io::Write};

pub static mut LOGGER: FileLogger = FileLogger { file: None };

pub struct FileLogger {
    pub file: Option<File>,
}

impl FileLogger {
    pub fn create(path: &str) {
        let file = File::create(path).unwrap();
        unsafe {
            LOGGER = FileLogger { file: Some(file) };
        };
    }

    pub fn info(&mut self, text: &str) {
        self.file.as_mut().unwrap().write(format!("[INF] {text}").as_bytes()).unwrap();
    }

    pub fn debug(&mut self, text: &str) {
        self.file.as_mut().unwrap().write(format!("[DBG] {text}").as_bytes()).unwrap();
    }

    pub fn error(&mut self, text: &str) {
        self.file.as_mut().unwrap().write(format!("[ERR] {text}").as_bytes()).unwrap();
    }
}
