use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::{self, write, DirEntry},
    io::{Error, ErrorKind},
    path::PathBuf,
};
use text_align::TextAlign;

#[derive(Serialize, Deserialize, Clone)]
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

    pub fn delete(name: &str, path_save_dir: PathBuf, all: bool) -> Result<(), Error> {
        let mut result = Ok(());

        if all {
            search_dir(&path_save_dir, |entry| {
                if let Err(e) = fs::remove_file(entry.path()) {
                    result = Err(e)
                }
            })
        } else {
            search_dir(&path_save_dir, |entry| {
                if let Some(entry_file_name) = entry.path().file_stem() {
                    if name == entry_file_name {
                        match fs::remove_file(entry.path()) {
                            Ok(_) => {}
                            Err(e) => result = Err(e),
                        }
                    }
                }
            })
        }

        result
    }

    pub fn toggle(name: &str, path_save_dir: &PathBuf, all: bool) -> Result<Option<Task>, Error> {
        let mut error: Option<Error> = None;
        if all {
            match Task::list(&path_save_dir, false, false) {
                Some(mut tasks) => {
                    tasks.iter_mut().for_each(|f| {
                        f.completed = !f.completed;

                        f.completed_at = if f.completed {
                            Some(Local::now())
                        } else {
                            None
                        };

                        if let Err(e) = f.save(true, path_save_dir) {
                            error = Some(e);
                        }
                    });
                    return Ok(None);
                }
                None => {
                    return Ok(None);
                }
            }
        }

        if let Some(e) = error {
            return Err(e);
        }

        match Task::find(name, path_save_dir) {
            Some(mut task_vec) => match task_vec.len() {
                0 => return Err(Error::new(ErrorKind::NotFound, "Task not found.")),

                1 => {
                    task_vec[0].completed = !task_vec[0].completed;
                    return Ok(Some(task_vec[0].clone()));
                }

                _ => {
                    println!("Found more than one task. Which one should be toggled?");
                    let iter = task_vec.clone().into_iter();
                    for (index, task) in iter.enumerate() {
                        println!("{}. {}", index, task);
                    }

                    let mut input = String::new();
                    match std::io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            let index = input.trim().parse::<usize>();
                            match index {
                                Ok(index) => {
                                    if index <= task_vec.len() - 1 {
                                        task_vec[index].completed = !task_vec[index].completed;
                                        return Ok(Some(task_vec[index].clone()));
                                    } else {
                                        return Err(Error::new(
                                            ErrorKind::InvalidInput,
                                            "Index input outside valid range.",
                                        ));
                                    }
                                }
                                Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e)),
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
            },
            None => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("No task found that contains '{}'", name),
                ));
            }
        }
    }

    pub fn find(task_name: &str, path_save: &PathBuf) -> Option<Vec<Task>> {
        let mut tasks: Vec<Task> = vec![];

        search_dir(path_save, |entry| {
            if let Some(file_name) = entry.path().file_stem() {
                if file_name.to_str().unwrap().contains(task_name) {
                    match read_encoded_file(entry.path()) {
                        Ok(task) => tasks.push(task),
                        Err(_) => {}
                    }
                }
            }
        });

        if tasks.len() > 0 {
            Some(tasks)
        } else {
            None
        }
    }

    pub fn list(path_save: &PathBuf, only_completed: bool, only_todo: bool) -> Option<Vec<Task>> {
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
            if only_completed {
                tasks.retain(|task| task.completed == true);
            } else if only_todo {
                tasks.retain(|task| task.completed == false)
            }
            return Some(tasks);
        }
    }

    pub fn save(&self, overwrite: bool, path_save: &PathBuf) -> Result<(), Error> {
        save_encoded_file(&self, overwrite, path_save)
    }
}

fn search_dir<F>(path_save_dir: &PathBuf, mut f: F)
where
    F: FnMut(DirEntry),
{
    if let Ok(entries) = fs::read_dir(path_save_dir) {
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

fn save_encoded_file(task: &Task, overwrite: bool, path_save_dir: &PathBuf) -> Result<(), Error> {
    if !&path_save_dir.is_dir() {
        fs::create_dir(&path_save_dir)?;
    }

    let mut path_to_save = path_save_dir.clone();
    path_to_save.push(&task.name);
    path_to_save.set_extension("tsk");

    let file: Vec<u8>;
    match bincode::serialize(&task) {
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        Ok(f) => file = f,
    }

    if overwrite {
        match write(path_to_save, file) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }

    match Task::find(&task.name, &path_save_dir) {
        None => match write(path_to_save, file) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        },

        Some(task_vec) => {
            if task_vec.iter().any(|i| &i.name == &task.name) {
                return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    "File already exists and overwrite is set to false.",
                ));
            } else {
                match write(path_to_save, file) {
                    Ok(_) => return Ok(()),
                    Err(e) => return Err(e),
                }
            }
        }
    }
}
