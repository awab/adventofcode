use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let mut lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    // this sorts it by day, but the guards starting in the 23:XX hour will be at the end...
    lines.sort();
    let mut date = "".to_string();
    let mut current_guard = 0;
    let mut sleep_minute = 0;
    let mut wake_minute = 0;

    let mut minutes_asleep = HashMap::new();
    let mut guards_minute_count = HashMap::new();
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let current_date = &parts[0][1..parts[0].len()];
        if current_date != date {
            date = current_date.to_string();
        } 

        let current_time_str = &parts[1][0..parts[1].len() - 1];
        let current_time_parts: Vec<_> = current_time_str.split(":").collect();
        let current_hour = current_time_parts[0].parse::<u16>().unwrap();
        let mut current_minute = current_time_parts[1].parse::<u16>().unwrap();
        if current_hour == 23 {
            // offset those before midnight as negative numbers
            current_minute = 60 - current_minute;
        }

        if parts[2] == "Guard" {
            // a new guard is coming on duty
            current_guard = parts[3][1..parts[3].len()].parse::<u16>().unwrap();
        } else if parts[2] == "falls" {
            sleep_minute = current_minute;
        } else if parts[2] == "wakes" {
            wake_minute = current_minute;

            // now that we have a guard id, a start time for the nap, and the duration, we can collect the data
            if sleep_minute < 0 {
                sleep_minute = 0;
            }

            // let's make sure we've noted the guard worked on this day
            let date_parts = date.split("-");
            let total_minutes_asleep = minutes_asleep.entry(current_guard).or_insert(0);
            *total_minutes_asleep += wake_minute - sleep_minute;

            let minutes_per_day = guards_minute_count.entry(current_guard).or_insert(HashMap::new());
            
            for index in sleep_minute..wake_minute {
                let specific_day = minutes_per_day.entry(index).or_insert(0);
                *specific_day += 1;
            }
        }
    }

    /* Part 1
    let guard_who_slept_most_minutes =     
        minutes_asleep
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .unwrap();

    let guard_calendar = guards_minute_count.get(guard_who_slept_most_minutes).unwrap();
    let minute_guard_slept_the_most =     
        guard_calendar
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .unwrap();

    println!("Result: {}", guard_who_slept_most_minutes * minute_guard_slept_the_most);
    */

    let mut guard_with_highest_minute = 0;
    let mut highest_minute_slept = 0;
    let mut highest_minute_slept_value = 0;

    for key in guards_minute_count.keys() {
        let guard_calendar = guards_minute_count.get(key).unwrap();
        let minute_guard_slept_the_most =     
            guard_calendar
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, _v)| k)
                .unwrap();

        let guards_max = guard_calendar.get(minute_guard_slept_the_most).unwrap();
        if *guards_max > highest_minute_slept_value {
            highest_minute_slept = *minute_guard_slept_the_most;
            highest_minute_slept_value = *guards_max;
            guard_with_highest_minute = *key;
        }
    }

    println!("Result: {}", guard_with_highest_minute as u32 * highest_minute_slept as u32);
}
