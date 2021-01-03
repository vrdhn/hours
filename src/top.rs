// just put serde(default) on everything ..
// let use upgrade from any version to any
use crate::color::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Task {
    #[serde(default)]
    id: usize,

    #[serde(default)]
    text: String,
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
        self.tasks.push(Task { id, text });
        id
    }

    fn task_show(&self, id: usize) {
        let task = &self.tasks[id - 1];
        println!("    {:5} {}", green(&task.id), blue(&task.text));
    }

    pub fn task_new_show(&mut self, text: &str) {
        let id = self.task_add(text);
        self.task_show(id);
    }

    pub fn task_list(&self) {
        for task in &self.tasks {
            println!("    {:>5} {}", green(&task.id), blue(&task.text));
        }
    }
}
