pub mod data;

fn main() {
    let calories_by_elf = read_to_calories_by_elf_reverse_sorted();
    // Part 1;
    println!("Max Calories Carried by A Single Elf: {}", calories_by_elf[0]);

    // Part 2;
    print!("Sum of Top 3 Elves:");
    let mut sum: u32 = 0;
    for n in 0..3 {
        sum = add_value(sum,calories_by_elf[n]);
        
    }
    println!(" {} calories", sum);
}

fn read_to_calories_by_elf_reverse_sorted() -> Vec::<u32>{
    let mut calories_by_elf = Vec::<u32>::new();
    let mut sum: u32 = 0;
    if let Ok(lines) = data::read_data_from_file() {
        for line in lines {
            if let Ok(calories) = line {
                if !calories.is_empty() {
                    sum = add_value(sum, calories.parse().unwrap());
                }
                else {
                    calories_by_elf.push(sum);
                    sum = 0;
                }
            }
        }
    }
    calories_by_elf.push(sum);

    calories_by_elf.sort_by(|a, b| b.cmp(a));

    // Return
    calories_by_elf
}

fn add_value(x: u32, value_to_add: u32) -> u32 {
    x + value_to_add
}

// Read Line by Line. While value, sum, then push.