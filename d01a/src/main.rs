use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let mut largest_inventory = 0;

    for block in input.split("\n\n") {
        let mut total: i32 = 0;

        for line in block.lines() {
            total += i32::from_str_radix(line, 10).unwrap();
        }

        if total > largest_inventory {
            largest_inventory = total;
        }
    }

    println!("largest_inventory: {largest_inventory}");
}
