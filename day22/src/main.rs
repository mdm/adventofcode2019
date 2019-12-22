use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Technique {
    DealNewStack,
    DealWithIncrement(usize),
    Cut(i64),
}

fn apply_technique(deck: Vec<u64>, technique: &Technique) -> Vec<u64> {
    match technique {
        Technique::DealNewStack => {
            let mut new_deck = deck.clone();
            new_deck.reverse();
            new_deck
        }
        Technique::DealWithIncrement(increment) => {
            let mut new_deck = deck.clone();
            for i in 0..deck.len() {
                let deck_len = new_deck.len();
                new_deck[i * increment % deck_len] = deck[i];
            }

            new_deck
        }
        Technique::Cut(cards) => {
            if *cards < 0 {
                let cards = -cards as usize;
                let mut new_deck = deck.clone();
                for i in 0..cards {
                    new_deck[i] = deck[deck.len() - cards + i];
                }
                for i in cards..deck.len() {
                    new_deck[i] = deck[i - cards];
                }
                new_deck
            } else {
                let cards = *cards as usize;
                let mut new_deck = deck.clone();
                for i in 0..cards {
                    new_deck[deck.len() - cards + i] = deck[i];
                }
                for i in cards..deck.len() {
                    new_deck[i - cards] = deck[i];
                }
                new_deck
            }
        }
    }
}

fn naive_shuffle(deck: &Vec<u64>, techniques: &Vec<Technique>) -> Vec<u64> {
    let mut deck = deck.clone();
    for technique in techniques {
        deck = apply_technique(deck, technique);
    }

    deck
}

fn track_card(deck_size: usize, card_index: usize, rounds: usize, techniques: &Vec<Technique>) -> usize {
    let mut techniques = (*techniques).clone();
    techniques.reverse();

    let mut card_index = card_index;
    // let mut history = HashMap::new();
    for _i in 0..rounds {
        for technique in &techniques {
            card_index = match technique {
                Technique::DealNewStack => deck_size - card_index - 1,
                Technique::DealWithIncrement(increment) => {
                    let mut tmp = card_index;
                    while tmp % increment > 0 {
                        let old_temp = tmp;
                        tmp += deck_size;
                        if old_temp > tmp {
                            dbg!(old_temp, tmp);
                        }
                    }
                    tmp / increment
                },
                Technique::Cut(cards) => {
                    if *cards < 0 {
                        let cards = -cards as usize;
                        if card_index < cards {
                            card_index + (deck_size - cards)
                        } else {
                            card_index - cards
                        }
                    } else {
                        let cards = *cards as usize;
                        if card_index < deck_size - cards {
                            card_index + cards
                        } else {
                            card_index - (deck_size - cards)
                        }
                    }
                },
            };
        }
    }

    return card_index;
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
                    if let Ok(increment) = tokens[tokens.len() - 1].parse::<usize>() {
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

    let mut deck = Vec::new();
    for i in 0..10007 {
        deck.push(i);
    }

    deck = naive_shuffle(&deck, &techniques);

    for (i, card) in deck.iter().enumerate() {
        if *card == 2019 {
            println!("{}", i);
            break;
        }
    }

    // println!("{}", track_card(10007, 3589, 1, &techniques));

    println!("{}", track_card(119315717514047, 2020, 101741582076661, &techniques));
}
