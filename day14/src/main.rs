use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
struct Chemical {
    name: String,
    amount: u64,
}

#[derive(Debug)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

fn producible_amount(a: u64, b: u64) -> (u64, u64) {
    let floor = a / b;
    if a % b == 0 {
        (floor, 0)
    } else {
        let ceil = floor + 1;
        let producible = ceil * b;
        (ceil, producible - a)
    }
}

fn produce(request: &Chemical, reactions: &HashMap<String, Reaction>, spares: &mut HashMap<String, u64>, depth: u8) -> u64 {
    // let indentation = (0..depth).map(|_| "  ").collect::<String>();

    // println!("{}Producing {} {}", &indentation, &request.amount, &request.name);
    // dbg!(&spares);

    if request.name == "ORE" {
        return request.amount;
    }

    let request_reaction = reactions.get(&request.name).unwrap();
    let actual_amount = producible_amount(request.amount, request_reaction.output.amount);
    if actual_amount.1 > 0 {
        // println!("{}Produced {} spare {}", &indentation, &actual_amount.1, &request.name);
    }
    *spares.entry(request.name.clone()).or_insert(0) += actual_amount.1;
    let mut total_ore = 0;
    for input in &request_reaction.inputs {
        if input.name == "ORE" {
            let total_input = Chemical {
                name: input.name.clone(),
                amount: input.amount * actual_amount.0,
            };
            total_ore += produce(&total_input, reactions, spares, depth + 1);
            continue;
        }

        let available_amount = *spares.entry(input.name.clone()).or_insert(0);
        if input.amount * actual_amount.0 > available_amount {
            let total_input = Chemical {
                name: input.name.clone(),
                amount: input.amount * actual_amount.0 - available_amount,
            };
            if available_amount > 0 {
                // println!("{}Using {} spare {}", &indentation, &available_amount, &input.name);
                *spares.entry(input.name.clone()).or_insert(0) = 0;
            }
            total_ore += produce(&total_input, reactions, spares, depth + 1);
        } else {
            *spares.entry(input.name.clone()).or_insert(0) -= input.amount * actual_amount.0;
        }
    }

    return total_ore;
}

fn main() {
    let regex = regex::Regex::new(r"(\d+) (\w+)").unwrap();
    let mut reactions = HashMap::new();
    for line in std::io::stdin().lock().lines() {
        let reaction = line.unwrap();
        let mut chemicals = Vec::new();
        for capture in regex.captures_iter(&reaction) {
            // dbg!(&capture);
            let chemical = Chemical {
                name: capture[2].to_string(),
                amount: capture[1].parse::<u64>().unwrap(),
            };
            chemicals.push(chemical);
        }

        // dbg!(&chemicals);

        let reaction = Reaction {
            output: chemicals.pop().unwrap(),
            inputs: chemicals,
        };

        // dbg!(&reaction);
        reactions.insert(reaction.output.name.clone(), reaction);
    }

    // dbg!(&reactions);

    let mut available = HashMap::new();
    let desired = Chemical {
        name: "FUEL".to_string(),
        amount: 1,
    };
    println!("{}", produce(&desired, &reactions, &mut available, 0));

    let mut factor = 100_000_000 as u64;
    let mut fuel = factor;
    loop {
        // dbg!(fuel);
        let mut available = HashMap::new();
        let desired = Chemical {
            name: "FUEL".to_string(),
            amount: fuel,
        };
        let ore_needed =  produce(&desired, &reactions, &mut available, 0);

        if ore_needed > 1_000_000_000_000 {
            if factor == 1 {
                break;
            }

            fuel -= factor;
            factor /= 10;
        }

        fuel += factor;
    }
    println!("{}", fuel - 1);
}
