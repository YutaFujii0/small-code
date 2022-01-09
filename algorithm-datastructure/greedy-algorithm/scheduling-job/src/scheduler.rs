use std::collections::VecDeque;

type Weight = u64;
type Length = u64;
type Job = (Weight, Length);
type Jobs = VecDeque::<Job>;

pub fn total_weighted_completion_time(jobs: &Jobs) -> u64 {
    let mut completion_time = 0u64;
    let mut w_completion_time = 0u64;
    for job in jobs {
        completion_time += job.1;
        w_completion_time += job.0 * completion_time;
    }
    w_completion_time
}

#[allow(dead_code)]
pub fn schedule_by_diff(jobs: Jobs) -> Jobs {
    merge_sort(jobs)
}

fn merge_sort(mut jobs: Jobs) -> Jobs {
    if jobs.len() <= 1 {
        return VecDeque::from([jobs[0]])
    }
    let half = jobs.len() / 2;
    let right = merge_sort(jobs.split_off(half));
    let left = merge_sort(jobs);
    merge(left, right)
}

fn merge(mut left: Jobs, mut right: Jobs) -> Jobs {
    let mut merged = VecDeque::new();
    while left.len() > 0 && right.len() > 0 {
        let comp_diff = (left[0].0 as i64 - left[0].1 as i64) - (right[0].0 as i64 - right[0].1 as i64);
        if comp_diff > 0 {
            merged.push_back(left.pop_front().unwrap())
        } else if comp_diff < 0 {
            merged.push_back(right.pop_front().unwrap())
        } else if left[0].0 > right[0].0 {
            merged.push_back(left.pop_front().unwrap())
        } else {
            merged.push_back(right.pop_front().unwrap())
        }
    }
    if left.len() > 0 {
        merged.append(&mut left);
    }
    if right.len() > 0 {
        merged.append(&mut right);
    }
    merged
}

#[allow(dead_code)]
pub fn schedule_by_ratio(jobs: Jobs) -> Jobs {
    merge_sort2(jobs)
}

fn merge_sort2(mut jobs: Jobs) -> Jobs {
    if jobs.len() <= 1 {
        return VecDeque::from([jobs[0]])
    }
    let half = jobs.len() / 2;
    let right = merge_sort2(jobs.split_off(half));
    let left = merge_sort2(jobs);
    merge2(left, right)
}

fn merge2(mut left: Jobs, mut right: Jobs) -> Jobs {
    let mut merged = VecDeque::new();
    while left.len() > 0 && right.len() > 0 {
        let comp_diff = (left[0].0 as f64 / left[0].1 as f64) - (right[0].0 as f64 / right[0].1 as f64);
        if comp_diff > 0.0 {
            merged.push_back(left.pop_front().unwrap())
        } else {
            merged.push_back(right.pop_front().unwrap())
        }
    }
    if left.len() > 0 {
        merged.append(&mut left);
    }
    if right.len() > 0 {
        merged.append(&mut right);
    }
    merged
}
