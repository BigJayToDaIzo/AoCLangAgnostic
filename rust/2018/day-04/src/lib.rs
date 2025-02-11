use chrono::{prelude::*, DateTime, TimeZone};
use std::collections::{BTreeMap, HashMap};

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut clock: BTreeMap<DateTime<Utc>, Event> = BTreeMap::new();
    for line in inp.lines() {
        let (dt, event) = parse_line(line);
        clock.insert(dt, event);
    }
    let mut sleep_logs: Vec<SleepLog> = Vec::new();
    for (date, event) in clock {
        match event {
            Event::BeginShift(id) => {
                sleep_logs.push(SleepLog {
                    id,
                    date,
                    sleep_start: 0,
                    sleep_grid: [0; 60],
                });
            }
            Event::Sleep => {
                let mut popped_log = sleep_logs.pop().unwrap();
                popped_log.sleep_start = date.minute();
                sleep_logs.push(popped_log);
            }
            Event::Wake => {
                let mut popped_log = sleep_logs.pop().unwrap();
                for minute in popped_log.sleep_start..=date.minute() {
                    popped_log.sleep_grid[minute as usize] += 1;
                }
                sleep_logs.push(popped_log);
            }
        }
    }
    // analyze sleep logs
    // build heat map of times slept per minute
    let mut heat_map: HashMap<u32, HeatMap> = HashMap::new();
    for log in sleep_logs {
        // build heatmap for log
        let mut ht_map = heat_map.entry(log.id).or_insert(HeatMap {
            id: log.id,
            heat_map: [0; 60],
            total_slept: 0,
        });
        // we have either a new or existing heatmap
        for i in 0..log.sleep_grid.len() {
            if log.sleep_grid[i] == 1 {
                ht_map.heat_map[i] += 1;
                ht_map.total_slept += 1;
            }
        }
    }
    // we're almost there, heat map poppin'

    "".to_string()
}

pub fn part_two(_inp: &str) -> String {
    "".to_string()
}

fn parse_line(line: &str) -> (DateTime<Utc>, Event) {
    // break up line and build parse out date and time
    // PrimitiveDateTime::new(date!(2019-01-01), time!(0:00))
    let line = line.replace("[", "").replace("]", "");
    let line: Vec<&str> = line.split_whitespace().collect();
    let date: Vec<&str> = line[0].split('-').collect();
    let date: Vec<u32> = date.iter().map(|x| x.parse().unwrap()).collect();
    let year: i32 = date[0] as i32;
    let time: Vec<&str> = line[1].split(':').collect();
    let time: Vec<u32> = time.iter().map(|x| x.parse().unwrap()).collect();
    let dt = Utc
        .with_ymd_and_hms(year, date[1], date[2], time[0], time[1], 0)
        .unwrap();
    let event: Event;
    if line.len() > 4 {
        let id: u32 = line[3].replace("#", "").parse().unwrap();
        event = Event::BeginShift(id);
    } else if line[2].contains("falls") {
        event = Event::Sleep;
    } else {
        event = Event::Wake;
    }
    (dt, event)
}

#[derive(Debug)]
struct HeatMap {
    id: u32,
    heat_map: [u32; 60],
    total_slept: u32,
}

#[derive(Debug)]
struct SleepLog {
    id: u32,
    date: DateTime<Utc>,
    sleep_start: u32,
    sleep_grid: [u32; 60],
}

#[derive(Debug)]
enum Event {
    Wake,
    Sleep,
    BeginShift(u32),
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = "";

        let res = part_one(&input);

        assert_eq!(res, "");
    }

    #[test]
    pub fn test_part_two() {
        let input = "";

        let res = part_two(&input);

        assert_eq!(res, "");
    }
}
