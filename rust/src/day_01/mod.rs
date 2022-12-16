use std::str::FromStr;

pub fn day_1() {
    let file = include_str!("input.txt");

    let mut result = file
        .split("\n\n")
        .map(|elf| {
            elf
                .split("\n")
                .map(|cal| u32::from_str(cal).unwrap_or(0))
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));
    let top = result.first().unwrap_or(&0);
    let top_3: u32 = result.iter().take(3).sum();
    println!("{:#?}", top);
    println!("{:#?}", top_3);
}
