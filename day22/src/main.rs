use std::io::BufRead;

const DECK_SIZE_PART_1: u64 = 10007;
const DECK_SIZE_PART_2: u64 = 119315717514047;

const NUM_SHUFFLES_PART_1: u64 = 1;
const NUM_SHUFFLES_PART_2: u64 = 101741582076661;

const STARTING_CARD_PART_1: u64 = 2019;
const STARTING_CARD_PART_2: u64 = 2020;

#[derive(Debug, Clone)]
enum Technique {
    DealNewStack,
    DealWithIncrement(u64),
    Cut(i64),
}

struct Shuffle {
    deck_size: u64,
    dependent: u64,
    independent: u64,
}

impl Shuffle {
    fn new(deck_size: u64) -> Shuffle {
        Shuffle {
            deck_size,
            dependent: 1,
            independent: 0,
        }
    }

    fn double(shuffle: &Shuffle) -> Shuffle {
        let dependent = (shuffle.dependent * shuffle.dependent) % shuffle.deck_size;
        let independent = (((shuffle.dependent * shuffle.independent) % shuffle.deck_size) + shuffle.independent) % shuffle.deck_size;

        Shuffle {
            deck_size: shuffle.deck_size,
            dependent,
            independent,
        }
    }

    fn add_constant(&mut self, n: u64) {
        self.independent += n;
        self.independent %= self.deck_size;
    }

    fn multiply_constant(&mut self, n: u64) {
        dbg!(self.dependent, n);
        self.dependent *= n;
        self.dependent %= self.deck_size;
        self.independent *= n;
        self.independent %= self.deck_size;
    }

    fn apply_technique(&mut self, technique: &Technique) {
        match technique {
            Technique::DealNewStack => {
                self.add_constant(1);
                self.multiply_constant(self.deck_size - 1);
            }
            Technique::DealWithIncrement(n) => {
                self.multiply_constant(*n);
            }
            Technique::Cut(n) => {
                self.add_constant((self.deck_size as i64 - n) as u64);
            }
        }
    }

    fn apply_shuffle(&mut self, other: &Shuffle) {
        self.dependent *= other.dependent;
        self.dependent %= self.deck_size;
        self.independent += (self.dependent * other.independent) % self.deck_size;
        self.independent %= self.deck_size;
    }

    fn evaluate(&self, x: u64) -> u64 {
        (((self.dependent * x) % self.deck_size) + self.independent) % self.deck_size
    }
}

fn main() {
    let mut techniques = Vec::new();
    for line in std::io::stdin().lock().lines() {
        if let Ok(technique) = line {
            let tokens = technique.split(" ").collect::<Vec<_>>();
            match tokens[tokens.len() - 2] {
                "new" => {
                    techniques.push(Technique::DealNewStack);
                }
                "increment" => {
                    if let Ok(increment) = tokens[tokens.len() - 1].parse::<u64>() {
                        techniques.push(Technique::DealWithIncrement(increment));
                    }
                }
                "cut" => {
                    if let Ok(cards) = tokens[tokens.len() - 1].parse::<i64>() {
                        techniques.push(Technique::Cut(cards));
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    let mut shuffle = Shuffle::new(DECK_SIZE_PART_1);
    for technique in techniques.iter() {
        shuffle.apply_technique(technique);
    }
    println!("{}", shuffle.evaluate(STARTING_CARD_PART_1));

    let mut shuffle = Shuffle::new(DECK_SIZE_PART_2);
    for technique in techniques.iter() {
        shuffle.apply_technique(technique);
    }
    let mut final_shuffle = Shuffle::new(DECK_SIZE_PART_2);
    let mut mask: u64 = 1;
    while mask < NUM_SHUFFLES_PART_2 {
        if (mask & NUM_SHUFFLES_PART_2) != 0 {
            final_shuffle.apply_shuffle(&shuffle);
        }

        shuffle = Shuffle::double(&shuffle);
        mask <<= 1;
    }
    println!("{}", final_shuffle.evaluate(STARTING_CARD_PART_2));
}
