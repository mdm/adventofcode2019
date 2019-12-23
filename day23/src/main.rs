use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Context {
    program: Vec<i64>,
    pc: usize,
    inputs: VecDeque<i64>,
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

fn run(mut context: &mut Context) -> Option<i64> {
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
                let input = match context.inputs.pop_front() {
                    Some(value) => value,
                    None => -1,
                };

                write(&mut context, 1, input);

                context.pc += 2;
                if input == -1 {
                    return None;
                }
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
        }
    }

    return None;
}

fn string2ints(string: &str) -> Vec<i64> {
    string.as_bytes().iter().map(|byte| *byte as i64).collect::<Vec<_>>()
}

fn ints2string(ints: &Vec<i64>) -> String {
    let bytes = ints.iter().map(|int| *int as u8).collect::<Vec<_>>();
    match String::from_utf8(bytes) {
        Ok(string) => string,
        Err(_) => String::new(),
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();
    let mut program = Vec::new();
    for token in input.split(",") {
        program.push(token.parse::<i64>().expect(token));
    }

    let mut contexts = Vec::new();
    let mut outputs = Vec::new();
    for i in 0..50 {
        let context = Context {
            program: program.clone(),
            pc: 0,
            inputs: VecDeque::new(),
            relative_base: 0,
            heap: HashMap::new(),
        };

        contexts.push(context);
        contexts[i].inputs.push_back(i as i64);

        outputs.push(VecDeque::new());
    }

    let mut nat_memory = None;
    let mut nat_last_y = None;
    'run: loop {
        let mut idle_counter = 0;
        for i in 0..50 {
            match run(&mut contexts[i]) {
                Some(output) => {
                    outputs[i].push_back(output);
                }
                None => {
                    idle_counter += 1;
                }
            }

            if outputs[i].len() >= 3 {
                if let Some(destination) = outputs[i].pop_front() {
                    if let Some(x) = outputs[i].pop_front() {
                        if let Some(y) = outputs[i].pop_front() {
                            // dbg!(destination, x, y);
                            if destination == 255 {
                                if nat_memory == None {
                                    println!("{}", y);
                                }

                                nat_memory = Some((x, y));
                                // break 'run;
                            } else {
                                contexts[destination as usize].inputs.push_back(x);
                                contexts[destination as usize].inputs.push_back(y);
                            }
                        };
                    };
                };
            }
        }

        if idle_counter == 50 {
            if let Some(packet) = nat_memory {
                contexts[0].inputs.push_back(packet.0);
                contexts[0].inputs.push_back(packet.1);

                if let Some(y) = nat_last_y {
                    if y == packet.1 {
                        println!("{}", y);
                        break;
                    }
                }

                nat_last_y = Some(packet.1);
            }
        }
    }
}
