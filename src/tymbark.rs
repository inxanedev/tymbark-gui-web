use std::ffi::CString;
use std::{fs, process::exit};

use rand::{prelude::ThreadRng, thread_rng};
use rand::seq::SliceRandom;
use winapi::um::winuser::{MessageBoxA, MB_OK};

pub struct TymbarkGenerator {
    wordlist: Vec<String>,
    rng: ThreadRng
}

fn read_wordlist<T: ToString>(filename: &T) -> Vec<String> {
    log::info!("Reading wordlist...");
    match fs::read_to_string(filename.to_string()) {
        Ok(s) => {
            let result = s.split('\n').map(|line| line.trim().to_owned()).collect();
            log::info!("Done reading wordlist!");
            result
        }
        Err(e) => {
            log::error!("Cannot open wordlist file! {}", e);
            
            let title = CString::new("Cannot find wordlist!").unwrap();
            let text = CString::new("Make sure there is a food.txt file next to the executable, or pass your own wordlist using the -f command line parameter.").unwrap();

            unsafe {
                MessageBoxA(std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), MB_OK);
            }

            exit(1);
        }
    }
}

impl TymbarkGenerator {
    pub fn new<T: ToString>(filename: &T) -> TymbarkGenerator {
        TymbarkGenerator {
            wordlist: read_wordlist(filename),
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