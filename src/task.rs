use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Read},
    path::PathBuf,
};

use crate::PATH_SAVE;

#[derive(Serialize, Deserialize)]
pub struct Task {
    name: String,
    completed: bool,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
}

impl Task {
    fn new(name: &str) -> Task {
        Task {
            name: name.to_string(),
            completed: false,
            created_at: Local::now(),
            completed_at: None,
        }
    }

    fn edit(name: &str) -> Result<Task, Error> {
        todo!()
    }

    fn delete(name: &str) -> Result<(), Error> {
        todo!()
    }

    fn toggle(name: &str) -> Result<(), Error> {
        todo!()
    }

    fn find(name: &str) -> Option<Task> {
        if let Ok(entries) = fs::read_dir(PATH_SAVE) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(f_type) = entry.file_type() {
                        if f_type.is_file() {
                            match read_encoded_file(entry.path()) {
                                Ok(tsk) => return Some(tsk),
                                Err(_) => return None,
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn list() {
        todo!()
    }
}

fn read_encoded_file(path: PathBuf) -> Result<Task, Error> {
    let file = fs::read(&path)
        .expect(format!("Couldn't open file at {}", path.to_str().unwrap()).as_str());

    let task: Result<Task, Box<bincode::ErrorKind>> = bincode::deserialize(&file);
    match task {
        Ok(tsk) => return Ok(tsk),
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    }
}

fn save_encoded_file(task: &Task) -> Result<(), Error> {
    let path = format!("{}{}.tsk", PATH_SAVE, task.name);
    let file = bincode::serialize(&task);

    match file {
        Ok(t) => match File::create(path) {
            Ok(_) => return Ok(()),
            Err(e) => Err(e),
        },
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    }
}
