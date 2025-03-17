mod task01;
mod task02;
mod task03;
mod task04;

trait TaskTrait {
    fn run() {
        println!("Implementing is in progress");
    }
}

pub fn run_all() {
    task01::Task::run();
    task02::Task::run();
    task03::Task::run();
    task04::Task::run();
}

pub fn run_last() {
    task04::Task::run();
}
