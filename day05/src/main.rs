fn fetch(program: &Vec::<i32>, pc: usize, offset: u32) -> i32 {
    let mode = program[pc] / (10i32.pow(offset + 1)) % 10;
    // dbg!(mode);

    match mode {
        0 => {
            let address = program[pc + offset as usize] as usize;
            return program[address];
        }
        1 => {
            return program[pc + offset as usize];
        }
        _ => panic!("Illegal mode flag."),
    }
}

fn run(mut program: Vec::<i32>) {
    let mut pc = 0;
    while program[pc] != 99 {
        // println!("{}: {}", pc, program[pc]);
        match program[pc] % 100 {
            1 => {
                let operand1 = fetch(&program, pc, 1);
                let operand2 = fetch(&program, pc, 2);
                let destination = program[pc + 3] as usize;

                program[destination] = operand1 + operand2;
                pc += 4;
            }
            2 => {
                let operand1 = fetch(&program, pc, 1);
                let operand2 = fetch(&program, pc, 2);
                let destination = program[pc + 3] as usize;

                program[destination] = operand1 * operand2;
                pc += 4;
            }
            3 => {
                let destination = program[pc + 1] as usize;

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                input = input.trim_end().to_string();
                program[destination] = input.parse::<i32>().unwrap();
                pc += 2;
            }
            4 => {
                let operand = fetch(&program, pc, 1);

                println!("{}", operand);
                pc += 2;
            }
            5 => {
                let operand1 = fetch(&program, pc, 1);
                let operand2 = fetch(&program, pc, 2);

                if operand1 != 0 {
                    pc = operand2 as usize;
                } else {
                    pc += 3;
                }
            }
            6 => {
                let operand1 = fetch(&program, pc, 1);
                let operand2 = fetch(&program, pc, 2);

                if operand1 == 0 {
                    pc = operand2 as usize;
                } else {
                    pc += 3;
                }
            }
            7 => {
                let operand1 = fetch(&program, pc, 1);
                let operand2 = fetch(&program, pc, 2);
                let destination = program[pc + 3] as usize;

                if operand1 < operand2 {
                    program[destination] = 1;
                } else {
                    program[destination] = 0;
                }
                pc += 4;
            }
            8 => {
                let operand1 = fetch(&program, pc, 1);
                let operand2 = fetch(&program, pc, 2);
                let destination = program[pc + 3] as usize;

                if operand1 == operand2 {
                    program[destination] = 1;
                } else {
                    program[destination] = 0;
                }
                pc += 4;
            }
            _ => panic!("Illegal operation."),
        };
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();
    let mut program = Vec::new();
    for token in input.split(",") {
        program.push(token.parse::<i32>().expect(token));
    }

    run(program);
}
