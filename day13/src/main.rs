use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i64,
    y: i64,
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
                let input = match context.inputs.pop_front() {
                    Some(value) => value,
                    None => 0,
                };

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

fn display(game: &HashMap::<Position, i64>) {

    let min_x = game.iter().fold(std::i64::MAX, |min, panel| {
        std::cmp::min(min, panel.0.x)
    });

    let max_x = game.iter().fold(std::i64::MIN, |max, panel| {
        std::cmp::max(max, panel.0.x)
    });

    let min_y = game.iter().fold(std::i64::MAX, |min, panel| {
        std::cmp::min(min, panel.0.y)
    });

    let max_y = game.iter().fold(std::i64::MIN, |max, panel| {
        std::cmp::max(max, panel.0.y)
    });

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    // dbg!(width, height);

    let snapshot = (0..height).map(|y| {
        (0..width).map(|x| {
            let position = Position { x: x - min_x, y: y - min_y };
            match game.get(&position) {
                Some(color) => {
                    match color {
                        1 => '#',
                        2 => 'O',
                        3 => '=',
                        4 => '*',
                        _ => ' ',
                    }
                }
                None => {
                    ' '
                }
            }
        }).collect::<String>()
    }).collect::<Vec<String>>().join("\n");

    println!("{}", snapshot);
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

    let mut game = HashMap::<Position, i64>::new();
    loop {
        let x = match run(&mut context) {
            Some(output) => {
                output
            }
            None => {
                break;
            }
        };

        let y = match run(&mut context) {
            Some(output) => {
                output
            }
            None => {
                break;
            }
        };

        let tile_id = match run(&mut context) {
            Some(output) => {
                output
            }
            None => {
                break;
            }
        };

        game.insert(Position { x, y }, tile_id);
    }

    println!("{}", game.iter().fold(0, |acc, (_position, tile_id)| {
        if *tile_id == 2 {
            acc + 1
        } else {
            acc
        }
    }));

    let mut context = Context {
        program: program.clone(),
        pc: 0,
        inputs: VecDeque::new(),
        relative_base: 0,
        heap: HashMap::new(),
    };
    context.program[0] = 2;

    let mut game = HashMap::<Position, i64>::new();
    let mut score = 0;
    let mut moves = 0;
    let mut ball_position: Option<Position> = None;
    let mut paddle_position: Option<Position> = None;
    let mut track_ball = false;
    loop {
        let x = match run(&mut context) {
            Some(output) => {
                output
            }
            None => {
                break;
            }
        };

        let y = match run(&mut context) {
            Some(output) => {
                output
            }
            None => {
                break;
            }
        };

        let tile_id = match run(&mut context) {
            Some(output) => {
                output
            }
            None => {
                break;
            }
        };

        if x == -1 && y == 0 {
            score = tile_id;
        } else {
            game.insert(Position { x, y }, tile_id);

            match tile_id {
                3 => {
                    paddle_position = Some(Position { x, y });
                }
                4 => {
                    ball_position = Some(Position { x, y });

                    if let (Some(paddle), Some(ball)) = (&paddle_position, &ball_position) {
                        match ball.x.cmp(&paddle.x) {
                            std::cmp::Ordering::Equal => {
                                track_ball = true;
                            }
                            std::cmp::Ordering::Less => {
                                if track_ball {
                                    context.inputs.push_back(-1);
                                }
                            }
                            std::cmp::Ordering::Greater => {
                                if track_ball {
                                    context.inputs.push_back(1);
                                }
                            }
                        }
                    }

                    moves += 1;

                    if moves % 100 == 0 {
                        // println!("{}", score);
                        // display(&game);
                    }
                }
                _ => {}
            }
        }
    }

    println!("{}", score);
}
