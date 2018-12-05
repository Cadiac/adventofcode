extern crate regex;
extern crate chrono;

use chrono::prelude::*;
use regex::Regex;

use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../input.txt");

struct Event {
    timestamp: DateTime<Utc>,
    text: String
}

fn part_1(file: &str) -> u32 {
    let mut events: Vec<Event> = Vec::new();
    let mut guard_sleep_log = HashMap::new();

    for line in file.lines() {
        let input_regex = Regex::new(r"\[(.+)\] (.+)$").unwrap();
        
        for capture in input_regex.captures_iter(line) {
            let event = Event{
                timestamp: Utc.datetime_from_str(&capture[1], "%Y-%m-%d %H:%M").expect("Invalid date"),
                text: String::from(&capture[2])
            };

            events.push(event);
        }
    }

    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let guard_regex = Regex::new(r".+#(\d+).+$").unwrap();
    let mut current_guard = 0;
    let mut fell_asleep_timestamp = Utc::now();

    for event in events {
        if event.text.contains("begins shift") {
            let guard_id = guard_regex
                .captures(&event.text)
                .expect("Invalid guard ID")[1]
                .parse::<u32>()
                .expect("Parse fail");
            current_guard = guard_id;
        } else if event.text.contains("falls asleep") {
            fell_asleep_timestamp = event.timestamp;
        } else if event.text.contains("wakes up") {
            let slept_duration = event.timestamp
                .signed_duration_since(fell_asleep_timestamp)
                .num_minutes() as u32;

            let mut slept_minutes = Vec::new();

            for minute in 0..slept_duration {
                slept_minutes.push(fell_asleep_timestamp.minute() + minute);
            }

            // Push the slept minutes to log
            guard_sleep_log.entry(current_guard).or_insert(Vec::new()).push(slept_minutes);
        }
    }

    let most_sleepy = guard_sleep_log
        .iter()
        .map(|(guard_id, log)| {
            let mut total_sleep: u32 = 0;
            for sleep in log {
                let minutes_slept = sleep.len() as u32;
                total_sleep += minutes_slept;
            }
            (guard_id, total_sleep)
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    println!("Most sleepy: {}", most_sleepy.0);

    let mut minute_frequencey: HashMap<u32,u32> = HashMap::new();

    for sleeps in guard_sleep_log[most_sleepy.0].iter() {
        for minute in sleeps {
            *minute_frequencey.entry(*minute).or_insert(0) += 1;
        }
    };

    let most_common_minute = minute_frequencey
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .expect("No winning minute");

    println!("Most common minute is {} with value {}", most_common_minute.0, most_common_minute.1);

    most_sleepy.0 * most_common_minute.0
}

fn part_2(file: &str) -> u32 {
    let mut events: Vec<Event> = Vec::new();
    
    // Lets make the data structure HashMap (by minute) containing
    // HashMaps (by guard ID) containing the frequency of sleeps on that minute
    // by that guard.
    let mut minute_sleep_log: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

    for line in file.lines() {
        let input_regex = Regex::new(r"\[(.+)\] (.+)$").unwrap();
        
        for capture in input_regex.captures_iter(line) {
            let event = Event{
                timestamp: Utc.datetime_from_str(&capture[1], "%Y-%m-%d %H:%M").expect("Invalid date"),
                text: String::from(&capture[2])
            };

            events.push(event);
        }
    }

    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let guard_regex = Regex::new(r".+#(\d+).+$").unwrap();
    let mut current_guard = 0;
    let mut fell_asleep_timestamp = Utc::now();

    for event in events {
        if event.text.contains("begins shift") {
            let guard_id = guard_regex
                .captures(&event.text)
                .expect("Invalid guard ID")[1]
                .parse::<u32>()
                .expect("Parse fail");
            current_guard = guard_id;
        } else if event.text.contains("falls asleep") {
            fell_asleep_timestamp = event.timestamp;
        } else if event.text.contains("wakes up") {
            let slept_duration = event.timestamp
                .signed_duration_since(fell_asleep_timestamp)
                .num_minutes() as u32;

            for minute in 0..slept_duration {
                let asleep_at_minute = fell_asleep_timestamp.minute() + minute;

                *minute_sleep_log
                    .entry(asleep_at_minute)
                    .or_insert(HashMap::new())
                    .entry(current_guard)
                    .or_insert(0) += 1;
            }
        }
    };

    // minute, id, freq
    let mut most_frequent: (u32, u32, u32) = (0, 0, 0);

    // I'm having bad time with iter max functions etc. Lets just use for loops
    for (&minute, guards) in &minute_sleep_log {
        // id, freq
        let mut most_frequent_guard_this_minute: (u32, u32) = (0, 0);
        
        for (&guard_id, &freq) in guards {
            if most_frequent_guard_this_minute.1 < freq {
                most_frequent_guard_this_minute = (guard_id, freq);
            }
        }

        if most_frequent_guard_this_minute.1 > most_frequent.2 {
            most_frequent = (minute, most_frequent_guard_this_minute.0, most_frequent_guard_this_minute.1);
        }
    }
    
    println!("Found most frequent: minute {}, id {}, freq {}", most_frequent.0, most_frequent.1, most_frequent.2);

    most_frequent.1 * most_frequent.0
}

fn main() {
    let part1_result = part_1(INPUT_FILE);
    let part2_result = part_2(INPUT_FILE);
 
    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE: &str = include_str!("../test/example.txt");

    #[test]
    fn it_solves_day04_part1_example() {
        assert_eq!(part_1(TEST_FILE), 240);
    }

    #[test]
    fn it_solves_day04_part2_example() {
        assert_eq!(part_2(TEST_FILE), 4455);
    }
}
