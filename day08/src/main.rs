fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let image = input.trim_end().to_string();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let width = input.trim_end().parse::<usize>().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let height = input.trim_end().parse::<usize>().unwrap();

    let mut min_zeros = width * height;
    let mut num_ones = 0;
    let mut num_twos = 0;
    let mut chars = image.chars();

    let mut decoded_image = Vec::new();
    for _y in 0..height {
        let mut row = Vec::new();
        for _x in 0..width {
            row.push(2);
        }
        decoded_image.push(row);
    }

    for _layer in 0..(image.len() / (width * height)) {
        let mut histogram = [0; 10];
        for pixel in 0..(width * height) {
            let digit = chars.next().unwrap().to_digit(10).unwrap() as usize;
            histogram[digit] += 1;

            let x = pixel % width;
            let y = pixel / width;

            if decoded_image[y][x] == 2 {
                decoded_image[y][x] = digit;
            }
        }

        if histogram[0] < min_zeros {
            min_zeros = histogram[0];
            num_ones = histogram[1];
            num_twos = histogram[2];
        }
    }

    println!("{}", num_ones * num_twos);

    let image_string = decoded_image.iter().map(|row| {
        row.iter().map(|pixel| {
            match pixel {
                0 => " ".to_string(),
                1 => "#".to_string(),
                _ => " ".to_string(),
            }
        }).collect::<Vec<String>>().join("")
    }).collect::<Vec<String>>().join("\n");

    println!("{}", image_string);
}
