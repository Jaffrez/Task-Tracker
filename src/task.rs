use chrono::{DateTime, Utc};
use clap::ValueEnum;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct Task {
    id: u64,
    name: String,
    description: String,
    status: Status,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl Task {
    pub fn new(id: u64, name: String, description: String) -> Self {
        Task {
            id,
            name,
            description,
            status: Status::Todo,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    pub fn updated(&self) -> &DateTime<Utc> {
        &self.updated
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.updated = Utc::now();
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.updated = Utc::now();
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
        self.updated = Utc::now();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Todo,
    Skip,
    InProgress,
    Done,
}

impl ValueEnum for Status {
    fn value_variants<'a>() -> &'a [Self] {
        &[Status::Todo, Status::Skip, Status::InProgress, Status::Done]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Status::Todo => Some(clap::builder::PossibleValue::new("todo")),
            Status::Skip => Some(clap::builder::PossibleValue::new("skip")),
            Status::InProgress => Some(clap::builder::PossibleValue::new("in_progress")),
            Status::Done => Some(clap::builder::PossibleValue::new("done")),
        }
    }
}

pub struct TaskVec {
    tasks: Vec<Task>,
    next_id: u64,
}

impl TaskVec {
    pub fn new() -> Self {
        TaskVec {
            tasks: vec![],
            next_id: 0,
        }
    }

    pub fn from(path: &str) -> Self {
        // TODO: 从数据库获取数据
        todo!()
    }

    pub fn to(path: &str) {
        // TODO: 存储数据到数据库
        todo!()
    }

    pub fn add(&mut self, name: &str, desc: &str) -> Option<&mut Task> {
        let task = Task::new(self.next_id, name.to_string(), desc.to_string());
        self.tasks.push(task);
        self.next_id += 1;
        self.tasks.last_mut()
    }

    pub fn del(&mut self, id: u64) -> Task {
        let index = self.tasks.iter().position(|task| task.id() == id).unwrap();
        self.tasks.remove(index)
    }

    pub fn update(
        &mut self,
        id: u64,
        name: Option<&str>,
        desc: Option<&str>,
    ) -> Result<&Task, Error> {
        let index = match self.tasks.iter().position(|task| task.id() == id) {
            Some(idx) => idx,
            None => return Err(Error::new(ErrorKind::NotFound, "task not found")),
        };
        let task = self.tasks.get_mut(index).unwrap();
        if let Some(name) = name {
            task.set_name(name.to_string());
        }
        if let Some(desc) = desc {
            task.set_description(desc.to_string());
        }
        Ok(task)
    }

    pub fn mark(&mut self, id: u64, status: &Status) -> Result<&Task, Error> {
        let index = match self.tasks.iter().position(|task| task.id() == id) {
            Some(idx) => idx,
            None => return Err(Error::new(ErrorKind::NotFound, "task not found")),
        };
        let task = self.tasks.get_mut(index).unwrap();
        task.set_status(status.clone());
        Ok(task)
    }

    pub fn list_by_status(&self, status: Option<&Status>) -> Vec<&Task> {
        match status {
            Some(s) => {
                return self
                    .tasks
                    .iter()
                    .filter(|task| task.status() == s)
                    .collect();
            }
            None => {
                return self.tasks.iter().collect();
            }
        }
    }
}
