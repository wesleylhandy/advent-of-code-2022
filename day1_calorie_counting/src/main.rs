pub mod data;

fn main() {
    max_calories();
}

fn max_calories() {
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
    println!("{}", calories_by_elf.iter().max().unwrap());
}

fn add_value(x: u32, value_to_add: u32) -> u32 {
    x + value_to_add
}

// Read Line by Line. While value, sum, then push.