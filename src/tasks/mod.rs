mod task01;
mod task02;
mod task03;
mod task04;

trait TaskTrait {
    fn run(&self) {
        println!("Implementing is in progress");
    }
}

pub struct TasksModule {
    tasks: Vec<Box<dyn TaskTrait>>,
}

impl TasksModule {
    pub fn create() -> Self {
        let mut tasks: Vec<Box<dyn TaskTrait>> = Vec::new();

        tasks.push(Box::new(task01::Task));
        tasks.push(Box::new(task02::Task));
        tasks.push(Box::new(task03::Task));
        tasks.push(Box::new(task04::Task));

        TasksModule {
            tasks
        }
    }

    pub fn run_all(&self) {
        for task in &self.tasks {
            task.run();
        }
    }

    pub fn run_last(&self) {
        let tasks_length = self.tasks.len();
        let last_task = &self.tasks[tasks_length - 1];
        last_task.run();
    }
}
