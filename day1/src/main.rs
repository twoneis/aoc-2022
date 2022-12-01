use std::fs;

fn main() {
    let inputs = fs::read_to_string("inputs.txt").expect("Error reading input file");
    let split_inputs = inputs.lines();

    let mut cals_per_elf: Vec<i32> = Vec::new();
    cals_per_elf.push(0);

    let mut current_elf = 0;

    for line in split_inputs {
        if !line.is_empty() {
            cals_per_elf[current_elf] += line.parse::<i32>().unwrap();
        } else {
            current_elf += 1;
            cals_per_elf.push(0);
        }
    }

    cals_per_elf.sort();

    println!(
        "The elf carrying the most calories is carrying {} calories",
        cals_per_elf[cals_per_elf.len() - 1]
    );

    println!(
        "The three elves carrying the most calories combined carry {} calories",
        (cals_per_elf[cals_per_elf.len() - 1]
            + cals_per_elf[cals_per_elf.len() - 2]
            + cals_per_elf[cals_per_elf.len() - 3])
    )
}
