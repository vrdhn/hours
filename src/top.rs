// just put serde(default) on everything ..
// let use upgrade from any version to any
use crate::color::*;

#[derive(Display, Serialize, Deserialize, Debug, PartialEq)]
enum Status {
    Todo,
    Done,
}

impl Default for Status {
    fn default() -> Status {
        Status::Todo
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Task {
    #[serde(default)]
    id: usize,

    #[serde(default)]
    text: String,

    #[serde(default)]
    status: Status,
}

impl Task {
    pub fn show(&self) {
        println!(
            "    {:>5}  [{:>5}]     {}",
            green(&self.id),
            cyan(&self.status),
            blue(&self.text)
        );
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Top {
    #[serde(default)]
    tasks: Vec<Task>,
}

impl Top {
    pub fn new() -> Top {
        Top { tasks: vec![] }
    }

    fn task_add(&mut self, text: &str) -> usize {
        let id = self.tasks.len() + 1;
        let text = text.to_string();
        self.tasks.push(Task {
            id,
            text,
            status: Status::Todo,
        });
        id
    }

    fn task_show(&self, id: usize) {
        self.tasks[id - 1].show();
    }

    pub fn task_new_show(&mut self, text: &str) {
        let id = self.task_add(text);
        self.task_show(id);
    }

    pub fn task_list(&self, all: &bool) {
        for task in &self.tasks {
            if *all || task.status == Status::Todo {
                task.show();
            }
        }
    }

    pub fn task_done(&mut self, id: &usize) {
        self.tasks[id - 1].status = Status::Done;
        self.task_show(*id);
    }
}
