use std::collections::BTreeMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let mut candidate_rooms: Vec<Room> = Vec::new();
    let mut rooms: Vec<Room> = Vec::new();
    for line in input.lines() {
        let mut encrypted_name = "".to_string();
        let mut v: Vec<&str> = line.split(&['-', '[', ']'][..]).collect();
        let checksum: String = v[v.len() - 2].to_string();
        v.pop();
        v.pop();
        let sector_id: u32 = v[v.len() - 1].parse().unwrap();
        v.pop();
        for section in v {
            encrypted_name += section;
        }
        candidate_rooms.push(Room {
            encrypted_name,
            checksum,
            sector_id,
        });
    }
    'iter_rooms: for room in candidate_rooms {
        let mut char_map = BTreeMap::new();
        for c in room.encrypted_name.chars() {
            let count = char_map.entry(c).or_insert(0);
            *count += 1;
        }
        let mut checksum_to_count = "".to_string();
        for c in room.checksum.chars() {
            let count = char_map.get(&c);
            match count {
                Some(n) => checksum_to_count += &n.to_string(),
                None => {
                    continue 'iter_rooms;
                }
            }
        }
        // check sorted hash_to_count first
        let mut place_counter: u32 = checksum_to_count.chars().next().unwrap() as u32;
        for c in checksum_to_count.chars() {
            let c_as_u32 = c as u32;
            if place_counter < c_as_u32 {
                continue 'iter_rooms;
            }
            place_counter = c_as_u32;
        }
        for c in room.checksum.chars() {
            char_map.remove_entry(&c);
        }
        let lowest_count_in_checksum = checksum_to_count
            .chars()
            .next_back()
            .unwrap()
            .to_digit(10)
            .unwrap();
        for (_, v) in char_map {
            if v > lowest_count_in_checksum {
                continue 'iter_rooms;
            }
        }
        rooms.push(room);
    }
    let mut sector_id_sum = 0;
    for room in rooms {
        sector_id_sum += room.sector_id;
    }
    dbg!(sector_id_sum);
}

struct Room {
    encrypted_name: String,
    checksum: String,
    sector_id: u32,
}
