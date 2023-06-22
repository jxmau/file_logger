
use std::{fs::{OpenOptions}, io::Write};
use log::{trace, info, error};
use chrono;

pub struct FileLogger{
    path: String,
    name: String
}

impl FileLogger {

    pub fn new(path: String, name: String) -> Self {
        Self {path , name } 
    }

    pub fn info(&self, log: String) {
        let log = format!(" {} | {} | INFO | {log}\n", now(), self.name);
        self.write(&log);
    }

    pub fn error(&self, log: String) {
        let log = format!("{} | {} | ERROR | {log}\n", now(), self.name);
        self.write(&log);
    }

    pub fn warning(&self, log: String) {
        let log = format!("{} | {} | WARNING | {log}\n", now(), self.name);
        self.write(&log);
    }

    fn write(&self, log: &str) {
        trace!("path: {:?} ", self.path);
        let mut file = match OpenOptions::new().create(true).append(true).open(&self.path) {
            Ok(f) => f,
            Err(e) => {
                error!("Fail to get file");
                trace!(" {e} ");
                return;
            }
        };
        trace!("File : {file:?} ");
        match file.write_all(log.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                error!("Fail to write file");
                trace!(" {e} ");
                ()
            }
        }
        info!("Log has been saved in file.")
    }


}


pub fn now() -> String {
    chrono::offset::Utc::now().to_string()
}