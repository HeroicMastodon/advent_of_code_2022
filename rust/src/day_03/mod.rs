use std::collections::HashMap;
use crate::read_lines;


pub fn day_3() {
    let char_value = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, char)| (char, ( idx + 1 ) as u32))
        .collect::<HashMap<char, u32>>();
    let file_name = "src/day_03/input.txt";
    let file = read_lines(file_name).unwrap_or("".to_string());
    let lines = file.split("\n");

    let vec = lines.collect::<Vec<_>>();
    let result: u32 = vec
        .as_slice()
        .chunks(3)
        .map(|chunk| {
            let (first, second, third) = (chunk[0], chunk[1], chunk[2]);
            let mut seen_count: HashMap<char, u32> = HashMap::new();

            let mut local_is_seen: HashMap<char, bool> = HashMap::new();
            for chr in first.chars() {
                if local_is_seen.contains_key(&chr) {
                    continue;
                } else {
                    local_is_seen.insert(chr, true);
                }

                let count = match seen_count.contains_key(&chr) {
                    true => { seen_count[&chr] + 1 }
                    false => { 1 }
                };

                seen_count.insert(chr, count);
            }

            let mut local_is_seen: HashMap<char, bool> = HashMap::new();
            for chr in second.chars() {
                if local_is_seen.contains_key(&chr) {
                    continue;
                } else {
                    local_is_seen.insert(chr, true);
                }
                let count = match seen_count.contains_key(&chr) {
                    true => { seen_count[&chr] + 1 }
                    false => { 1 }
                };

                seen_count.insert(chr, count);
            }

            let mut local_is_seen: HashMap<char, bool> = HashMap::new();
            for chr in third.chars() {
                if local_is_seen.contains_key(&chr) {
                    continue;
                } else {
                    local_is_seen.insert(chr, true);
                }

                let count = match seen_count.contains_key(&chr) {
                    true => { seen_count[&chr] + 1 }
                    false => { 1 }
                };

                if count == 3 {
                    return char_value[&chr];
                }

                seen_count.insert(chr, count);
            }

            println!("Could not find match in chunk {chunk:#?}");
            0
        })
        .sum();

    // .map(|sack| {
    //     let sack_size = sack.chars().count();
    //     let compartment_size = sack_size / 2;
    //     let first = sack.chars().take(compartment_size);
    //     
    //     for first_item in first {
    //         let second = sack.chars().skip(compartment_size);
    //         for second_item in second {
    //             if first_item == second_item {
    //                 return char_value[&first_item];
    //             }
    //         }
    //     }
    // 
    //     println!("Couldn't find match in sack {sack:#?}");
    //     0
    // })
    // .sum();

    println!("{result:#?}");
}
