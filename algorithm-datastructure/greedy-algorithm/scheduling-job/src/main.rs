// const FILE_PATH: &str = "./dataset-sample.txt";
const FILE_PATH: &str = "./dataset.txt";

mod parse;
mod scheduler;
use self::parse::*;
use self::scheduler::*;

fn main() {
    println!("Scheduling jobs!");
    if let Ok(res) = parse(FILE_PATH) {
        let mut jobs_scheduled = schedule_by_diff(res.clone());
        let mut result = total_weighted_completion_time(&jobs_scheduled);
        println!("{:?}", result);

        jobs_scheduled = schedule_by_ratio(res.clone());
        result = total_weighted_completion_time(&jobs_scheduled);
        println!("{:?}", result);
    }
}
