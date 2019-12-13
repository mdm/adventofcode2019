use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

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
                let input = context.inputs.pop_front().unwrap();
                write(&mut context, 1, input);

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
        }
    }

    return None;
}

fn turn(orientation: &Direction, turn_direction: i64) -> Direction {
    return match turn_direction {
        0 => match orientation {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
        1 => match orientation {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
        _ => (*orientation).clone(),
    }
}

fn move_forward(position: &Position, orientation: &Direction) -> Position {
    return match orientation {
        Direction::Up => Position { x: position.x, y: position.y - 1 },
        Direction::Right => Position { x: position.x + 1, y: position.y },
        Direction::Down => Position { x: position.x, y: position.y + 1 },
        Direction::Left => Position { x: position.x - 1, y: position.y },
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

    let mut context = Context {
        program: program.clone(),
        pc: 0,
        inputs: VecDeque::new(),
        relative_base: 0,
        heap: HashMap::new(),
    };

    let mut position = Position { x: 0, y: 0 };
    let mut orientation = Direction::Up;
    let mut panels = HashMap::<Position, i64>::new();
    loop {
        match panels.get(&position) {
            Some(color) => {
                context.inputs.push_back(*color);
            }
            None => {
                context.inputs.push_back(0);
            }
        }

        let color = match run(&mut context) {
            Some(output) => {
                output
            }
            None => {
                break;
            }
        };

        let turn_direction = match run(&mut context) {
            Some(output) => {
                output
            }
            None => {
                break;
            }
        };

        panels.insert(position.clone(), color);

        orientation = turn(&orientation, turn_direction);
        position = move_forward(&position, &orientation);
    }

    println!("{}", panels.len());

    // part 2

        let mut context = Context {
        program: program.clone(),
        pc: 0,
        inputs: VecDeque::new(),
        relative_base: 0,
        heap: HashMap::new(),
    };

    let mut position = Position { x: 0, y: 0 };
    let mut orientation = Direction::Up;
    let mut panels = HashMap::<Position, i64>::new();
    panels.insert(position.clone(), 1);
    loop {
        match panels.get(&position) {
            Some(color) => {
                context.inputs.push_back(*color);
            }
            None => {
                context.inputs.push_back(0);
            }
        }

        let color = match run(&mut context) {
            Some(output) => {
                output
            }
            None => {
                break;
            }
        };

        let turn_direction = match run(&mut context) {
            Some(output) => {
                output
            }
            None => {
                break;
            }
        };

        panels.insert(position.clone(), color);

        orientation = turn(&orientation, turn_direction);
        position = move_forward(&position, &orientation);
    }

    let min_x = panels.iter().fold(std::i64::MAX, |min, panel| {
        std::cmp::min(min, panel.0.x)
    });

    let max_x = panels.iter().fold(std::i64::MIN, |max, panel| {
        std::cmp::max(max, panel.0.x)
    });

    let min_y = panels.iter().fold(std::i64::MAX, |min, panel| {
        std::cmp::min(min, panel.0.y)
    });

    let max_y = panels.iter().fold(std::i64::MIN, |max, panel| {
        std::cmp::max(max, panel.0.y)
    });

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    // dbg!(width, height);

    let registration = (0..height).map(|y| {
        (0..width).map(|x| {
            let position = Position { x: x - min_x, y: y - min_y };
            match panels.get(&position) {
                Some(color) => {
                    match color {
                        1 => '#',
                        _ => ' ',
                    }
                }
                None => {
                    ' '
                }
            }
        }).collect::<String>()
    }).collect::<Vec<String>>().join("\n");

    println!("{}", registration);
}
