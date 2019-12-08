use permutohedron::Heap;

#[derive(Debug)]
struct Context {
    program: Vec::<i32>,
    pc: usize,
    phase: i32,
    initialized: bool,
}

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

fn run(context: &mut Context, input: i32) -> Option<i32> {
    while context.program[context.pc] != 99 {
        // println!("{}: {}", context.pc, context.program[context.pc]);
        match context.program[context.pc] % 100 {
            1 => {
                let operand1 = fetch(&context.program, context.pc, 1);
                let operand2 = fetch(&context.program, context.pc, 2);
                let destination = context.program[context.pc + 3] as usize;

                context.program[destination] = operand1 + operand2;
                context.pc += 4;
            }
            2 => {
                let operand1 = fetch(&context.program, context.pc, 1);
                let operand2 = fetch(&context.program, context.pc, 2);
                let destination = context.program[context.pc + 3] as usize;

                context.program[destination] = operand1 * operand2;
                context.pc += 4;
            }
            3 => {
                let destination = context.program[context.pc + 1] as usize;

                if !context.initialized {
                    context.program[destination] = context.phase;
                    context.initialized = true;
                } else {
                    context.program[destination] = input;
                }
                context.pc += 2;
            }
            4 => {
                let operand = fetch(&context.program, context.pc, 1);

                context.pc += 2;
                return Some(operand);
            }
            5 => {
                let operand1 = fetch(&context.program, context.pc, 1);
                let operand2 = fetch(&context.program, context.pc, 2);

                if operand1 != 0 {
                    context.pc = operand2 as usize;
                } else {
                    context.pc += 3;
                }
            }
            6 => {
                let operand1 = fetch(&context.program, context.pc, 1);
                let operand2 = fetch(&context.program, context.pc, 2);

                if operand1 == 0 {
                    context.pc = operand2 as usize;
                } else {
                    context.pc += 3;
                }
            }
            7 => {
                let operand1 = fetch(&context.program, context.pc, 1);
                let operand2 = fetch(&context.program, context.pc, 2);
                let destination = context.program[context.pc + 3] as usize;

                if operand1 < operand2 {
                    context.program[destination] = 1;
                } else {
                    context.program[destination] = 0;
                }
                context.pc += 4;
            }
            8 => {
                let operand1 = fetch(&context.program, context.pc, 1);
                let operand2 = fetch(&context.program, context.pc, 2);
                let destination = context.program[context.pc + 3] as usize;

                if operand1 == operand2 {
                    context.program[destination] = 1;
                } else {
                    context.program[destination] = 0;
                }
                context.pc += 4;
            }
            _ => panic!("Illegal operation."),
        };
    }

    return None;
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();
    let mut program = Vec::new();
    for token in input.split(",") {
        program.push(token.parse::<i32>().expect(token));
    }

    let mut phases = (0..5).collect::<Vec<i32>>();
    let mut max_output = 0;
    for permutation in Heap::new(&mut phases) {
        let mut input = 0;

        let mut contexts = Vec::new();
        for amplifier in 0..5 {
            contexts.push(Context {
                program: program.clone(),
                pc: 0,
                phase: permutation[amplifier],
                initialized: false
            });
        }

        for amplifier in 0..5 {
            if let Some(output) = run(&mut contexts[amplifier], input) {
                    input = output;
            }
        }

        if input > max_output {
            max_output = input;
        }
    }

    println!("{}", max_output);

    let mut phases = (0..5).collect::<Vec<i32>>();
    let mut max_output = 0;
    for permutation in Heap::new(&mut phases) {
        let mut input = 0;

        let mut contexts = Vec::new();
        for amplifier in 0..5 {
            contexts.push(Context {
                program: program.clone(),
                pc: 0,
                phase: permutation[amplifier] + 5,
                initialized: false
            });
        }

        let mut amplifier = 0;
        loop {
            match run(&mut contexts[amplifier % 5], input) {
                Some(output) => {
                    input = output;
                }
                None => {
                    break;
                }
            }

            amplifier += 1;
        }

        if input > max_output {
            max_output = input;
        }
    }

    println!("{}", max_output);
}
