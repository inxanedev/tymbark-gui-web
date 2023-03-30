use rand::{prelude::ThreadRng, thread_rng};
use rand::seq::SliceRandom;

pub struct TymbarkGenerator {
    wordlist: Vec<String>,
    rng: ThreadRng
}

fn parse_wordlist(word_list: &str) -> Vec<String> {
    word_list.split('\n').map(|line| line.trim().to_owned()).collect()
}

impl TymbarkGenerator {
    pub fn new(food_list: &str) -> TymbarkGenerator {
        TymbarkGenerator {
            wordlist: parse_wordlist(food_list),
            rng: thread_rng()
        }
    }

    pub fn generate(&mut self, nouns: u32) -> String {
        let mut result = String::from("Tymbark ");

        for i in 0..nouns {
            result.push_str(self.wordlist.choose(&mut self.rng).unwrap());

            if i != nouns - 1 {
                result.push(' ');
            }
        }

        result
    }
}