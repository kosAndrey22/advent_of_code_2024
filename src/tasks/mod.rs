mod task01;
mod task02;
mod task03;

trait TaskTrait {
    fn run() {
        println!("Implementing is in progress");
    }
}

pub fn run_all() {
    task01::Task::run();
    task02::Task::run();
    task03::Task::run();
}
