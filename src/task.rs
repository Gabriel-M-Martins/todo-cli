use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::{self, DirEntry, File},
    io::{Error, ErrorKind},
    path::PathBuf,
};

use crate::PATH_SAVE;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub completed: bool,
    pub created_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let completed = if self.completed { "V" } else { "F" };
        let mut completed_date = String::new();

        if let Some(date) = self.completed_at {
            completed_date = String::from("Completion: ");
            completed_date.push_str(&date.format("%d/%m/%Y").to_string());
        }

        writeln!(
            f,
            "{} - {}\n  Creation: {} | {}",
            self.name,
            completed,
            self.created_at.format("%d/%m/%Y").to_string(),
            completed_date
        )
    }
}

impl Task {
    pub fn new(name: &str) -> Task {
        Task {
            name: name.to_string(),
            completed: false,
            created_at: Local::now(),
            completed_at: None,
        }
    }

    pub fn edit(name: &str) -> Result<Task, Error> {
        todo!()
    }

    pub fn delete(name: &str) -> Result<(), Error> {
        todo!()
    }

    pub fn toggle(name: &str) -> Result<(), Error> {
        todo!()
    }

    pub fn find(name: &str) -> Option<Task> {
        let mut task: Option<Task> = None;
        search_dir(|entry| {
            if entry.file_name().to_str().unwrap() == name {
                match read_encoded_file(entry.path()) {
                    Ok(tsk) => task = Some(tsk),
                    Err(_) => {}
                }
            }
        });

        None
    }

    pub fn list() -> Result<Vec<String>, Error> {
        let vec: Vec<String> = vec![];
        if let Ok(entries) = fs::read_dir(PATH_SAVE) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(f_type) = entry.file_type() {
                        if f_type.is_file() {}
                    }
                }
            }
        }
        todo!()
    }

    pub fn save(&self, overwrite: bool) -> Result<(), Error> {
        save_encoded_file(&self, overwrite)
    }
}

fn search_dir<F>(mut f: F)
where
    F: FnMut(DirEntry),
{
    if let Ok(entries) = fs::read_dir(PATH_SAVE) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(f_type) = entry.file_type() {
                    if f_type.is_file() {
                        f(entry)
                    }
                }
            }
        }
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

fn save_encoded_file(task: &Task, overwrite: bool) -> Result<(), Error> {
    let path = format!("{}{}.tsk", PATH_SAVE, task.name);
    let file = bincode::serialize(&task);

    let mut file_exists = false;
    search_dir(|f| {
        if f.file_name().to_str().unwrap() == &task.name {
            file_exists = true
        }
    });

    if (!file_exists) || (overwrite) {
        match file {
            Ok(t) => match File::create(path) {
                Ok(_) => return Ok(()),
                Err(e) => Err(e),
            },
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        }
    } else {
        Err(Error::new(
            ErrorKind::AlreadyExists,
            "File already exists and overwrite is set to false.",
        ))
    }
}
