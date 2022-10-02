use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Read},
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
        let files = fs::read_dir(PATH_SAVE).ok()?;

        for file in files {
            let file_name = file.ok()?.file_name();
            if name == file_name.to_str()? {
                let tsk = read_encoded_file(name).expect("Some error ocurred while reading file.");
                return Some(tsk);
            }
        }

        None
    }

    fn list() {
        todo!()
    }
}

fn read_encoded_file(name: &str) -> Result<Task, Error> {
    let path = format!("{}{}.tsk", PATH_SAVE, name);
    let file = fs::read(&path).expect(format!("Couldn't open file at {}", &path).as_str());

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
