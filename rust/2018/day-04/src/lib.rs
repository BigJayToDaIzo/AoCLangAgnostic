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
    let sleep_logs = build_sleep_logs(clock);
    let heat_map = build_heat_map(&sleep_logs);
    // find id of biggest sleeper
    let mut max_id = 0;
    let mut slept_max = 0;
    for (id, map) in &heat_map {
        if map.total_slept > slept_max {
            slept_max = map.total_slept;
            max_id = *id;
        }
    }
    // index (minute) of the peak of guards heatmap
    let sleepiest_elf_hm = &heat_map.get(&max_id).unwrap();
    let mut sleepiest_minute = 0;
    for min in 0..sleepiest_elf_hm.heat_map.len() {
        if sleepiest_elf_hm.heat_map[min] > sleepiest_elf_hm.heat_map[sleepiest_minute] {
            sleepiest_minute = min;
        }
    }
    (sleepiest_minute as u32 * max_id).to_string()
}

pub fn part_two(inp: &str) -> String {
    let mut clock: BTreeMap<DateTime<Utc>, Event> = BTreeMap::new();
    for line in inp.lines() {
        let (dt, event) = parse_line(line);
        clock.insert(dt, event);
    }
    let sleep_logs = build_sleep_logs(clock);
    let heat_map = build_heat_map(&sleep_logs);
    let mut map_peak_count = 0;
    let mut map_peak_minute = 0;
    let mut map_peak_id = 0;
    for (i, map) in heat_map {
        for minute in 0..map.heat_map.len() {
            if map.heat_map[minute] > map_peak_count {
                map_peak_count = map.heat_map[minute];
                map_peak_minute = minute;
                map_peak_id = i;
            }
        }
    }
    (map_peak_minute as u32 * map_peak_id).to_string()
}

fn build_heat_map(sleep_logs: &Vec<SleepLog>) -> HashMap<u32, HeatMap> {
    let mut heat_map: HashMap<u32, HeatMap> = HashMap::new();
    for log in sleep_logs {
        let ht_map = heat_map.entry(log.id).or_insert(HeatMap {
            heat_map: [0; 60],
            total_slept: 0,
        });
        for i in 0..log.sleep_grid.len() {
            if log.sleep_grid[i] == 1 {
                ht_map.heat_map[i] += 1;
                ht_map.total_slept += 1;
            }
        }
    }
    heat_map
}

fn build_sleep_logs(clock: BTreeMap<DateTime<Utc>, Event>) -> Vec<SleepLog> {
    let mut sleep_logs: Vec<SleepLog> = Vec::new();
    for (date, event) in clock {
        match event {
            Event::BeginShift(id) => {
                sleep_logs.push(SleepLog {
                    id,
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
    sleep_logs
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
    heat_map: [u32; 60],
    total_slept: u32,
}

#[derive(Debug)]
struct SleepLog {
    id: u32,
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
        let input = read_lines();

        let res = part_one(&input);

        assert_eq!(res, "21083");
    }

    #[test]
    pub fn test_part_two() {
        let input = read_lines();

        let res = part_two(&input);

        assert_eq!(res, "53024");
    }
}
