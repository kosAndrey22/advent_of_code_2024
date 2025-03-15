use tasks::TaskTrait;

pub mod tasks;

fn main() {
    println!("Starting advent of code");
    tasks::task01::Task::run();
}
