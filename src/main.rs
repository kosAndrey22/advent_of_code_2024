use std::env;

pub mod tasks;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut run_all = false;

    for arg in args {
        match arg.as_str() {
            "-a" => run_all = true,
            _ => {}
        }
    }

    let mut task_module = tasks::TasksModule::create();

    if run_all {
        println!("Starting advent of code");
        task_module.run_all();
        println!("End!");
        return;
    }

    println!("Running last task");
    task_module.run_last();
    println!("End!");
}
