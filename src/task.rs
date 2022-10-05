use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::{self, write, DirEntry},
    io::{Error, ErrorKind},
    path::PathBuf,
};
use text_align::TextAlign;

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

        let completed_date: String;
        if let Some(date) = self.completed_at {
            completed_date = format!("Completion: {}", date.format("%d/%m/%Y, %H:%M").to_string())
        } else {
            completed_date = String::default();
        }

        let name = format!("{}", self.name).left_align(70);
        let completed = completed.center_align(25);
        let dates = format!(
            "Creation: {} | {}",
            self.created_at.format("%d/%m/%Y, %H:%M").to_string(),
            completed_date
        )
        .right_align(20);

        write!(f, "{}{}{}", name, completed, dates)
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

    pub fn delete(name: &str, path_save: PathBuf) -> Result<(), Error> {
        let mut result: Result<(), Error> = Err(Error::new(ErrorKind::NotFound, "Task not found."));

        search_dir(path_save, |entry| {
            if entry.file_name().to_str().unwrap().contains(name) {
                match fs::remove_file(entry.path()) {
                    Ok(_) => result = Ok(()),
                    Err(e) => result = Err(e),
                }
            }
        });

        result
    }

    pub fn toggle(name: &str, path_save: PathBuf) -> Result<Task, Error> {
        match Task::find(name, path_save) {
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

    pub fn find(task_name: &str, path_save: PathBuf) -> Option<Task> {
        let mut task: Option<Task> = None;
        search_dir(path_save, |entry| {
            // if entry.file_name().to_str().unwrap().contains(name) {
            //     match read_encoded_file(entry.path()) {
            //          Ok(tsk) => task = Some(tsk),
            //          Err(_) => {}
            //     }
            // }
            if let Some(file_name) = entry.path().file_stem() {
                if file_name == task_name {
                    match read_encoded_file(entry.path()) {
                        Ok(tsk) => task = Some(tsk),
                        Err(_) => {}
                    }
                }
            }
        });

        task
    }

    pub fn list(path_save: PathBuf) -> Option<Vec<Task>> {
        let mut files: Vec<PathBuf> = vec![];
        let mut tasks: Vec<Task> = vec![];
        search_dir(path_save, |entry| {
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

    pub fn save(&self, overwrite: bool, path_save: PathBuf) -> Result<(), Error> {
        save_encoded_file(&self, overwrite, path_save)
    }
}

fn search_dir<F>(path_save: PathBuf, mut f: F)
where
    F: FnMut(DirEntry),
{
    if let Ok(entries) = fs::read_dir(path_save) {
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

fn save_encoded_file(task: &Task, overwrite: bool, path_save_dir: PathBuf) -> Result<(), Error> {
    if !&path_save_dir.is_dir() {
        fs::create_dir(&path_save_dir)?;
    }

    let mut path_to_save = path_save_dir.clone();
    path_to_save.push(&task.name);
    path_to_save.set_extension("tsk");

    let file = bincode::serialize(&task);

    if overwrite {
        match file {
            Ok(t) => match write(path_to_save, &t) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        }
    } else if let None = Task::find(&task.name, path_save_dir.clone()) {
        match file {
            Ok(t) => match write(path_to_save, &t) {
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
