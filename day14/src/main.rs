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

fn produce(chemical: &Chemical, reactions: &HashMap<String, Reaction>, available: &mut HashMap<String, u64>) -> u64 {
    // println!("Producing {} {}", &chemical.amount, &chemical.name);

    if chemical.name == "ORE" {
        *available.entry(chemical.name.clone()).or_insert(0) += chemical.amount;
        return chemical.amount;
    }

    let reaction = reactions.get(&chemical.name).unwrap();
    let reactions_needed = (chemical.amount as f32 / reaction.output.amount as f32).ceil() as u64;
    *available.entry(chemical.name.clone()).or_insert(0) += reaction.output.amount * reactions_needed;
    return (0..reactions_needed).fold(0, |acc, i| {
        if chemical.name == "FUEL" && i % 1000 == 0{
            dbg!(i);
        }
        acc + reaction.inputs.iter().fold(0, |acc, input| {
            let mut ore_used = 0;
            let available_amount = *available.entry(input.name.clone()).or_insert(0);
            if  available_amount < input.amount {
                let missing_amount = input.amount - available_amount;
                ore_used = produce(&Chemical { name: input.name.clone(), amount: missing_amount }, reactions, available);
            }
            // dbg!(input);
            *available.entry(input.name.clone()).or_insert(0) -= input.amount;
            acc + ore_used
        })
    });
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

    let mut available = HashMap::new();
    let desired = Chemical {
        name: "FUEL".to_string(),
        amount: 1,
    };
    println!("{}", produce(&desired, &reactions, &mut available));

    let mut factor = 100_000_000 as u64;
    let mut fuel = factor;
    loop {
        dbg!(fuel);
        let mut available = HashMap::new();
        let desired = Chemical {
            name: "FUEL".to_string(),
            amount: fuel,
        };
        let ore_needed =  produce(&desired, &reactions, &mut available);

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
