fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    let range: Vec<&str> = input.split("-").collect();
    let start = range[0].parse::<u32>().unwrap();
    let end = range[1].parse::<u32>().unwrap() + 1;

    let mut num_valid1 = 0;
    let mut num_valid2 = 0;
    for candidate in start..end {
        let mut last_digit = candidate % 10;
        let mut other_digits = candidate / 10;

        let mut double = false;
        let mut run_length = 1;
        let mut true_double = false;
        let mut decreasing = false;
        while other_digits > 0 {
            let new_last_digit = other_digits % 10;

            if new_last_digit == last_digit {
                double = true;
                run_length += 1;
            } else {
                if run_length == 2 {
                    true_double = true;
                }
                run_length = 1;
            }

            if new_last_digit > last_digit {
                decreasing = true;
                break;
            }

            last_digit = new_last_digit;
            other_digits = other_digits / 10;
        }

        if run_length == 2 {
            true_double = true;
        }

        if double && !decreasing {
            num_valid1 += 1;

            if true_double {
                num_valid2 += 1;
            }
        }
    }

    println!("{}", num_valid1);
    println!("{}", num_valid2);
}
