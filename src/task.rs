use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::{self, write, DirEntry},
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
        let completed = if self.completed {
            "Concluído."
        } else {
            "A fazer."
        };
        let mut completed_date = String::new();

        if let Some(date) = self.completed_at {
            completed_date = String::from("| Completion: ");
            completed_date.push_str(&date.format("%d/%m/%Y").to_string());
        }

        write!(
            f,
            "{} - {}\n  Creation: {} {}",
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

    pub fn delete(name: &str) -> Result<(), Error> {
        let mut result: Result<(), Error> = Err(Error::new(ErrorKind::NotFound, "Task not found."));

        search_dir(|entry| {
            if entry.file_name().to_str().unwrap().contains(name) {
                match fs::remove_file(entry.path()) {
                    Ok(_) => result = Ok(()),
                    Err(e) => result = Err(e),
                }
            }
        });

        result
    }

    pub fn toggle(name: &str) -> Result<Task, Error> {
        match Task::find(name) {
            Some(mut task) => {
                if !task.completed {
                    task.completed = true;
                    task.completed_at = Some(Local::now())
                } else {
                    task.completed = false;
                }

                return Ok(task);
            }

            None => return Err(Error::new(ErrorKind::NotFound, "Task not found.")),
        }
    }

    pub fn find(name: &str) -> Option<Task> {
        let mut task: Option<Task> = None;
        search_dir(|entry| {
            if entry.file_name().to_str().unwrap().contains(name) {
                match read_encoded_file(entry.path()) {
                    Ok(tsk) => task = Some(tsk),
                    Err(_) => {}
                }
            }
        });

        task
    }

    pub fn list() -> Option<Vec<Task>> {
        let mut files: Vec<PathBuf> = vec![];
        let mut tasks: Vec<Task> = vec![];
        search_dir(|entry| {
            files.push(entry.path());
        });

        for path in files {
            if let Ok(task) = read_encoded_file(path) {
                tasks.push(task);
            }
        }

        if tasks.len() == 0 {
            return None;
        } else {
            return Some(tasks);
        }
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
        if f.file_name().to_str().unwrap().contains(&task.name) {
            file_exists = true
        }
    });

    if !file_exists || overwrite {
        match file {
            Ok(t) => match write(path, &t) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
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
