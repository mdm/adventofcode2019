fn run(mut program: Vec::<usize>) -> usize {
    let mut pc = 0;
    while program[pc] != 99 {
        let source1 = program[pc + 1];
        let source2 = program[pc + 2];
        let destination = program[pc + 3];

        program[destination] = match program[pc] {
            1 => program[source1] + program[source2],
            2 => program[source1] * program[source2],
            _ => panic!("Illegal operation."),
        };

        pc += 4;
    }

    // dbg!(program);

    return program[0];
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();
    let mut program = Vec::new();
    for token in input.split(",") {
        program.push(token.parse::<usize>().expect(token));
    }

    program[1] = 12;
    program[2] = 2;

    let result = run(program.clone());

    println!("{}", result);

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            program[1] = noun;
            program[2] = verb;
            if run(program.clone()) == 19690720 {
                println!("{}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}
