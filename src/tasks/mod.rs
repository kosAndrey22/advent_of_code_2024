mod task01;
mod task02;
mod task03;
mod task04;

trait TaskTrait {
    fn run(&mut self) {
        println!("Implementing is in progress");
    }
}

pub struct TasksModule {
    tasks: Vec<Box<dyn TaskTrait>>,
}

impl TasksModule {
    pub fn create() -> Self {
        let mut tasks: Vec<Box<dyn TaskTrait>> = Vec::new();

        tasks.push(Box::new(task01::Task::create()));
        tasks.push(Box::new(task02::Task::create()));
        tasks.push(Box::new(task03::Task::create()));
        tasks.push(Box::new(task04::Task::create()));

        TasksModule {
            tasks
        }
    }

    pub fn run_all(&mut self) {
        for task in &mut self.tasks {
            task.run();
        }
    }

    pub fn run_last(&mut self) {
        let tasks_length = self.tasks.len();
        let last_task = &mut self.tasks[tasks_length - 1];
        last_task.as_mut().run();
    }
}
