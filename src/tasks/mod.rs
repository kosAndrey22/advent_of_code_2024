mod task01;

trait TaskTrait {
    fn run() {
        println!("Implementing is in progress");
    }
}

pub fn run_all() {
    task01::Task::run();
}
