use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut inventories = Vec::new();

    for block in input.split("\n\n") {
        let mut total: i32 = 0;

        for line in block.lines() {
            total += i32::from_str_radix(line, 10).unwrap();
        }

        inventories.push(total);
    }

    inventories.sort();
    inventories.reverse();

    println!("{} {} {}", inventories.get(0).unwrap(), inventories.get(1).unwrap(), inventories.get(2).unwrap());
    println!("{}", inventories.get(0).unwrap() + inventories.get(1).unwrap() + inventories.get(2).unwrap());
}
