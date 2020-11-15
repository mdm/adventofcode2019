use std::io::BufRead;

const DECK_SIZE_PART_1: i128 = 10007;
const DECK_SIZE_PART_2: i128 = 119315717514047;

const NUM_SHUFFLES_PART_1: i128 = 1;
const NUM_SHUFFLES_PART_2: i128 = 101741582076661;

const STARTING_CARD_PART_1: i128 = 2019;
const FINAL_CARD_PART_2: i128 = 2020;

#[derive(Debug, Clone)]
enum Technique {
    DealNewStack,
    DealWithIncrement(i128),
    Cut(i128),
}

struct Shuffle {
    deck_size: i128,
    dependent: i128,
    independent: i128,
}

impl Shuffle {
    fn new(deck_size: i128) -> Shuffle {
        Shuffle {
            deck_size,
            dependent: 1,
            independent: 0,
        }
    }

    fn add_constant(&mut self, n: i128) {
        self.independent += n;
        self.independent = self.independent.rem_euclid(self.deck_size);
    }

    fn multiply_constant(&mut self, n: i128) {
        // dbg!(self.dependent, n);
        self.dependent *= n;
        self.dependent = self.dependent.rem_euclid(self.deck_size);
        self.independent *= n;
        self.independent = self.independent.rem_euclid(self.deck_size);
    }

    fn apply_technique(&mut self, technique: &Technique) {
        match technique {
            Technique::DealNewStack => {
                self.multiply_constant(-1);
                self.add_constant(self.deck_size - 1);
                // self.add_constant(1);
                // self.multiply_constant(self.deck_size - 1);
            }
            Technique::DealWithIncrement(n) => {
                self.multiply_constant(*n);
            }
            Technique::Cut(n) => {
                self.add_constant(self.deck_size - n);
            }
        }
    }

    fn apply_shuffle(&mut self, other: &Shuffle) {
        self.dependent *= other.dependent;
        self.dependent = self.dependent.rem_euclid(self.deck_size);
        self.independent = (self.independent * other.dependent).rem_euclid(self.deck_size) + other.independent;
        self.independent = self.independent.rem_euclid(self.deck_size);
    }

    fn double(&mut self) {
        self.independent += (self.independent * self.dependent).rem_euclid(self.deck_size);
        self.independent = self.independent.rem_euclid(self.deck_size);
        self.dependent *= self.dependent;
        self.dependent = self.dependent.rem_euclid(self.deck_size);
    }

    fn evaluate(&self, x: i128) -> i128 {
        (((self.dependent * x).rem_euclid(self.deck_size)) + self.independent).rem_euclid(self.deck_size)
    }

    fn evaluate_inverse(&self, x: i128) -> i128 {
        let (mut old_r, mut r) = (self.dependent, self.deck_size);
        let (mut old_s, mut s) = (1, 0);
        let (mut old_t, mut t) = (0, 1);

        while r != 0 {
            let quotient = old_r / r;
            let new_r = old_r - quotient * r;
            old_r = r;
            r = new_r;
            let new_s = old_s - quotient * s;
            old_s = s;
            s = new_s;
            let new_t = old_t - quotient * t;
            old_t = t;
            t = new_t;
        }

        let inverse = old_s.rem_euclid(self.deck_size);
        ((x - self.independent).rem_euclid(self.deck_size) * inverse).rem_euclid(self.deck_size)
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
                    if let Ok(increment) = tokens[tokens.len() - 1].parse::<i128>() {
                        techniques.push(Technique::DealWithIncrement(increment));
                    }
                }
                "cut" => {
                    if let Ok(cards) = tokens[tokens.len() - 1].parse::<i128>() {
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
    for _ in 0..NUM_SHUFFLES_PART_1 {
        for technique in techniques.iter() {
            shuffle.apply_technique(technique);
        }
    }
    println!("{}", shuffle.evaluate(STARTING_CARD_PART_1));

    let mut shuffle = Shuffle::new(DECK_SIZE_PART_2);
    for technique in techniques.iter() {
        shuffle.apply_technique(technique);
    }
    let mut final_shuffle = Shuffle::new(DECK_SIZE_PART_2);
    let mut mask: i128 = 1;
    while mask <= NUM_SHUFFLES_PART_2 {
        if (mask & NUM_SHUFFLES_PART_2) != 0 {
            final_shuffle.apply_shuffle(&shuffle);
        }

        shuffle.double();
        mask <<= 1;
    }
    println!("{}", final_shuffle.evaluate_inverse(FINAL_CARD_PART_2));
}
