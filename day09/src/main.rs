use std::collections::HashMap;

#[derive(Debug)]
struct Context {
    program: Vec::<i64>,
    pc: usize,
    phase: i64,
    initialized: bool,
    relative_base: usize,
    heap: HashMap<usize, i64>,
}

fn read_helper(context: &mut Context, address: usize) -> i64 {
    if address >= context.program.len() {
        match context.heap.get(&address) {
            Some(value) => {
                return *value;
            }
            None => {
                context.heap.insert(address, 0);
                return 0;
            }
        }
    } else {
        return context.program[address];
    }
}

fn write_helper(context: &mut Context, address: usize, value: i64) {
    if address >= context.program.len() {
        context.heap.insert(address, value);
    } else {
        context.program[address] = value;
    }
}

fn read(context: &mut Context, offset: u32) -> i64 {
    let mode = context.program[context.pc] / (10i64.pow(offset + 1)) % 10;
    // dbg!(mode);

    match mode {
        0 => {
            let address = context.program[context.pc + offset as usize] as usize;
            return read_helper(context, address);
        }
        1 => {
            return context.program[context.pc + offset as usize];
        }
        2 => {
            let address = context.program[context.pc + offset as usize];
            return read_helper(context, (context.relative_base as i64 + address) as usize);
        }
       _ => panic!("Illegal mode flag."),
    }
}

fn write(context: &mut Context, offset: u32, value: i64) {
    let mode = context.program[context.pc] / (10i64.pow(offset + 1)) % 10;

    match mode {
        0 => {
            let address = context.program[context.pc + offset as usize] as usize;
            write_helper(context, address, value);
        }
        2 => {
            let address = context.program[context.pc + offset as usize];
            write_helper(context, (context.relative_base as i64 + address) as usize, value);
        }
       _ => panic!("Illegal mode flag."),
    }
}

fn run(mut context: &mut Context, input: i64) -> Option<i64> {
    while context.program[context.pc] != 99 {
        // println!("{}: {}", context.pc, context.program[context.pc]);
        match context.program[context.pc] % 100 {
            1 => {
                let operand1 = read(&mut context, 1);
                let operand2 = read(&mut context, 2);
                write(&mut context, 3, operand1 + operand2);

                context.pc += 4;
            }
            2 => {
                let operand1 = read(&mut context, 1);
                let operand2 = read(&mut context, 2);
                write(&mut context, 3, operand1 * operand2);

                context.pc += 4;
            }
            3 => {
                if !context.initialized {
                    let phase = context.phase;
                    write(&mut context, 1, phase);
                    context.initialized = true;
                } else {
                    write(&mut context, 1, input);
                }

                context.pc += 2;
            }
            4 => {
                let operand = read(&mut context, 1);

                context.pc += 2;
                return Some(operand);
            }
            5 => {
                let operand1 = read(&mut context, 1);
                let operand2 = read(&mut context, 2);

                if operand1 != 0 {
                    context.pc = operand2 as usize;
                } else {
                    context.pc += 3;
                }
            }
            6 => {
                let operand1 = read(&mut context, 1);
                let operand2 = read(&mut context, 2);

                if operand1 == 0 {
                    context.pc = operand2 as usize;
                } else {
                    context.pc += 3;
                }
            }
            7 => {
                let operand1 = read(&mut context, 1);
                let operand2 = read(&mut context, 2);

                if operand1 < operand2 {
                    write(&mut context, 3, 1);
                } else {
                    write(&mut context, 3, 0);
                }
                context.pc += 4;
            }
            8 => {
                let operand1 = read(&mut context, 1);
                let operand2 = read(&mut context, 2);

                if operand1 == operand2 {
                    write(&mut context, 3, 1);
                } else {
                    write(&mut context, 3, 0);
                }
                context.pc += 4;
            }
            9 => {
                let operand = read(&mut context, 1);

                context.relative_base = (context.relative_base as i64 + operand) as usize;
                context.pc += 2;
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
        program.push(token.parse::<i64>().expect(token));
    }

    let mut input = 0;

    let mut context = Context {
        program: program.clone(),
        pc: 0,
        phase: 1,
        initialized: false,
        relative_base: 0,
        heap: HashMap::new(),
    };

    loop {
        match run(&mut context, input) {
            Some(output) => {
                println!("{}", output);
            }
            None => {
                break;
            }
        }
    }

    let mut context = Context {
        program: program.clone(),
        pc: 0,
        phase: 2,
        initialized: false,
        relative_base: 0,
        heap: HashMap::new(),
    };

    loop {
        match run(&mut context, input) {
            Some(output) => {
                println!("{}", output);
            }
            None => {
                break;
            }
        }
    }
}
