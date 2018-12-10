extern crate regex;
use std::collections::HashSet;
use std::collections::BTreeMap;
use regex::Regex;

const INPUT_FILE: &str = include_str!("../input.txt");

#[derive(Clone)]
struct Task {
    id: char,
    depends_on: HashSet<char>,
    work_total: u32,
    work_complete: u32
}

struct Worker {
    id: u32,
    active: bool,
    task: Option<Task>
}

fn find_next_available_task(all_tasks: &BTreeMap<char, Task>) -> Option<(char)> {
    let next_task = all_tasks
        .iter()
        .filter(|(_id, task)| task.depends_on.is_empty())
        .next();

    match next_task {
        Some((id, _task)) => Some(*id),
        None => None
    }
}

fn calculate_task_duration(task_id: char, min_duration: u32) -> u32 {
    (task_id as u8 - 'A' as u8) as u32 + 1 + min_duration
}

fn read_and_parse_file(file: &str, min_duration: u32) -> BTreeMap<char, Task> {
    let mut available_tasks: BTreeMap<char, Task> = BTreeMap::new();
    let input_regex = Regex::new(r"^Step (.+) must be finished before step (.+) can begin.$").unwrap();
    
    for line in file.lines() {
        for capture in input_regex.captures_iter(line) {
            let dependency = capture[1].parse::<char>().unwrap();
            let task_id = capture[2].parse::<char>().unwrap();

            available_tasks
                .entry(task_id)
                .or_insert(Task{
                    id: task_id,
                    depends_on: HashSet::new(),
                    work_total: calculate_task_duration(task_id, min_duration),
                    work_complete: 0u32
                })
                .depends_on
                .insert(dependency.clone());

            // Make sure the dependency task is also created
            available_tasks
                .entry(dependency.clone())
                .or_insert(Task{
                    id: dependency,
                    depends_on: HashSet::new(),
                    work_total: calculate_task_duration(dependency, min_duration),
                    work_complete: 0u32
                });
        };
    };

    available_tasks
}

fn part_1(file: &str) -> String {
    let mut all_tasks: BTreeMap<char, Task> = read_and_parse_file(file, 0);
    let mut task_order: Vec<String> = Vec::new();

    loop {
        let next_task_id = find_next_available_task(&all_tasks);

        match next_task_id {
            Some(next_task_id) => {
                task_order.push(next_task_id.to_string());
                for (_id, task) in all_tasks.iter_mut() {
                    task.depends_on.remove(&next_task_id.clone());
                };
                all_tasks.remove(&next_task_id);
            },
            None => {
                return task_order.join("");
            }
        };
    }
}

fn part_2(file: &str, concurrency: u32, min_duration: u32) -> u32 {
    let mut available_tasks: BTreeMap<char, Task> = read_and_parse_file(file, min_duration);
    let mut complete_tasks: Vec<char> = Vec::new();
    let mut workers: Vec<Worker> = Vec::new();
    let mut elapsed_time: u32 = 0;

    for worker in 0..concurrency {
        workers.push(Worker{
            id: worker,
            active: false,
            task: None
        })
    }

    loop {
        if available_tasks.len() == 0 && workers.iter().all(|worker| !worker.active) {
            return elapsed_time;
        }

        for worker in workers.iter_mut() {
            if !worker.active {
                let next_task_id = find_next_available_task(&available_tasks);

                match next_task_id {
                    Some(next_task_id) => {
                        println!("{:04} [worker.{}] Starting to work on {}", elapsed_time, worker.id, next_task_id);
                        let task = available_tasks.get(&next_task_id).expect("Task should exist").clone();
                        worker.task = Some(task);
                        worker.active = true;
                        available_tasks.remove(&next_task_id);
                    },
                    None => println!("{:04} [worker.{}] No tasks available, sleeping", elapsed_time, worker.id)
                };
            }
        }

        for worker in workers.iter_mut() {
            if worker.active {
                match worker.task {
                    Some(ref mut task) => {
                        task.work_complete += 1;
                        println!("{:04} [worker.{}] Task {}: {}/{}.", elapsed_time, worker.id, task.id, task.work_complete, task.work_total);
                        if task.work_complete >= task.work_total {
                            println!("{:04} [worker.{}] Task {} complete.", elapsed_time, worker.id, task.id);
                            complete_tasks.push(task.id);
                            for (_id, available_task) in available_tasks.iter_mut() {
                                available_task.depends_on.remove(&task.id);
                            };
                            worker.active = false;
                        }
                    }
                    None => println!("{:04} [worker.{}] No task", elapsed_time, worker.id)
                }
            }
        }

        elapsed_time += 1;
    }
}

fn main() {
    let part1_result = part_1(INPUT_FILE);
    let part2_result = part_2(INPUT_FILE, 5, 60);
    
    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE: &str = include_str!("../test/example.txt");

    #[test]
    fn it_solves_day07_part1_example() {
        assert_eq!(part_1(TEST_FILE), "CABDFE");
    }

    #[test]
    fn it_solves_day07_part2_example() {
        assert_eq!(part_2(TEST_FILE, 2, 0), 15);
    }

    #[test]
    fn it_calculates_task_duration() {
        assert_eq!(calculate_task_duration('A', 0), 1);
        assert_eq!(calculate_task_duration('C', 0), 3);
        assert_eq!(calculate_task_duration('Z', 0), 26);
        assert_eq!(calculate_task_duration('A', 60), 61);
        assert_eq!(calculate_task_duration('Z', 60), 86);
    }
}
