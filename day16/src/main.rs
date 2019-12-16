use std::collections::HashMap;

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}

fn extract_digits(signal: &Vec<i32>, skip_count: usize, take_count: usize) -> String {
    return signal.iter().skip(skip_count).take(take_count).map(|token|
        std::char::from_digit(*token as u32, 10).unwrap()
    ).collect::<String>();
}

fn pattern(in_position: usize, out_positon: usize) -> i32 {
    let base = vec!(0, 1, 0, -1);
    // dbg!(in_position, out_positon);
    return base[in_position / out_positon % base.len()];
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();
    let mut signal = Vec::new();
    for token in input.chars() {
        signal.push(token.to_digit(10).expect("Token is not a digit.") as i32);
    }

    let mut real_signal = signal.clone();
    for phase in 0..100 {
        if phase % 10 == 0 {
            println!("{}", extract_digits(&real_signal, 0, 8));
        }

        let new_signal = real_signal.iter().enumerate().map(|(i, _)| {
            // dbg!(signal.iter().enumerate().map(|(j, token)| { pattern(j + 1, i + 1) }).collect::<Vec<i32>>());

            real_signal.iter().enumerate().fold(0, |acc, (j, token)| {
                // dbg!((token, pattern(j + 1, i + 1)));
                acc + token * pattern(j + 1, i + 1)
            }).abs() % 10
        }).collect::<Vec<i32>>();

        real_signal = new_signal;
    }

    println!("{}\n", extract_digits(&real_signal, 0, 8));

    let message_offset = extract_digits(&signal, 0, 7).parse::<usize>().unwrap() + 1;
    real_signal = std::iter::repeat(&signal).take(10000).fold(vec!(0), |mut acc, s| {
        acc.extend(s);
        return acc;
    });
    dbg!(real_signal.len() - message_offset);

    for phase in 0..100 {
        if phase % 1 == 0 {
            println!("{}...", phase);
            // println!("{}", extract_digits(&real_signal, 1, 100));
        }

        let mut new_signal = vec!(0);
        for i in 1..real_signal.len() {
            let pattern_size = i * 4;
            let mut output = 0;

            for j in ((real_signal.len() - message_offset) / pattern_size)..(real_signal.len() / pattern_size + 1) {
                output += real_signal.iter()
                .skip(j * pattern_size + i)
                .take(i)
                .fold(0, |acc, token| {
                    acc + token
                });

                output += real_signal.iter()
                .skip(j * pattern_size + 3 * i)
                .take(i)
                .fold(0, |acc, token| {
                    acc - token
                });
            }

            new_signal.push(output.abs() % 10);
        }

        // dbg!(&real_signal, &new_signal);
        real_signal = new_signal;
    }

    println!("{}", extract_digits(&real_signal, message_offset, 100));
}
