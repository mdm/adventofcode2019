fn extract_digits(signal: &Vec<i32>, count: usize) -> String {
    return signal.iter().take(count).map(|token|
        std::char::from_digit(*token as u32, 10).unwrap()
    ).collect::<String>();
}

fn pattern(in_position: usize, out_positon: usize) -> i32 {
    let base = vec!(0, 1, 0, -1);
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
        // if phase % 10 == 0 {
        //     println!("{}", extract_digits(&real_signal, 8));
        // }

        let new_signal = real_signal.iter().enumerate().map(|(i, _)| {
            real_signal.iter().enumerate().fold(0, |acc, (j, token)| {
                acc + token * pattern(j + 1, i + 1)
            }).abs() % 10
        }).collect::<Vec<i32>>();

        real_signal = new_signal;
    }

    println!("{}", extract_digits(&real_signal, 8));

    let message_offset = extract_digits(&signal, 7).parse::<usize>().unwrap();
    real_signal = std::iter::repeat(&signal)
    .take(10000)
    .fold(Vec::new(), |mut acc, s| {
        acc.extend(s);
        return acc;
    });
    real_signal = real_signal.into_iter().skip(message_offset).collect::<Vec<i32>>();

    for phase in 0..100 {
        // if phase % 10 == 0 {
        //     println!("{}", extract_digits(&real_signal, 8));
        // }

        let mut new_signal = Vec::new();
        let mut output = 0;
        for i in 0..real_signal.len() {
            output += real_signal[real_signal.len() - i - 1];
            new_signal.push(output.abs() % 10);
        }

        new_signal.reverse();
        real_signal = new_signal;
    }

    println!("{}", extract_digits(&real_signal, 8));
}
