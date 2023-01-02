fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Could not read file");
    let mut result: i32 = 0;

    for line in contents.lines() {
        let (compartment_a, compartment_b) = line.split_at(line.len() / 2);
        let mut common_letter: char = 'a';

        'namaiwa: for char_a in compartment_a.chars() {
            for char_b in compartment_b.chars() {
                if char_a == char_b {
                    common_letter = char_a;
                    break 'namaiwa;
                }
            }
        }

        let mut score: i32 = 1;

        if common_letter.is_uppercase() {
            let difference_from_a = common_letter as i32 - 'A' as i32;
            score = 27;
            score += difference_from_a;
        } else {
            let difference_from_a = common_letter as i32 - 'a' as i32;
            score += difference_from_a;
        }

        result += score;
    }

    println!("result: {}", result);
}
