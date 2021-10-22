mod wordlist;

use std::io::{self, Write};
use wordlist::WORDLIST;

type Wordbits = u16;

fn get_word_vector() -> Vec<String> {
    WORDLIST
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn word_to_bits(word: &String) -> Result<Wordbits, String> {
    let word_vec = get_word_vector();
    for i in 0..word_vec.len() {
        if &word_vec[i] == word {
            return Ok(i as Wordbits);
        }
    }
    Err(format!("Unknown word: \"{}\"", word))
}

fn bits_to_word(bits: &Wordbits) -> String {
    get_word_vector()[*bits as usize].clone()
}

fn xor_wordlists(a: &Vec<Wordbits>, b: &Vec<Wordbits>) -> Vec<Wordbits> {
    (0..a.len())
        .map(|i| (a[i] ^ b[i]) % (1 << 11))
        .collect::<Vec<Wordbits>>()
}

fn get_a_b_x(words: Vec<Wordbits>) -> (Vec<Wordbits>, Vec<Wordbits>, Vec<Wordbits>) {
    let a: Vec<Wordbits> = words[0..words.len() / 2].to_vec();
    let b: Vec<Wordbits> = words[words.len() / 2..words.len()].to_vec();
    let x = xor_wordlists(&a, &b);
    (a, b, x)
}

fn input_word_bits(index: u16) -> Wordbits {
    loop {
        print!("Enter word {:02} > ", index);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let word = input.split_whitespace().collect::<Vec<&str>>()[0];
                let res = word_to_bits(&word.to_string());
                match res {
                    Err(e) => println!("{}", e),
                    Ok(v) => return v,
                }
            }
            Err(error) => println!("Error while readling from command line: {}", error),
        }
    }
}

fn create() {
    println!("");
    println!("=================");
    println!("Create redundancy");
    println!("=================");
    println!("");

    let mut word_list = Vec::new();
    for i in 1..25 {
        word_list.push(input_word_bits(i));
    }

    let (a, b, x) = get_a_b_x(word_list);
    println!("");
    println!("== Block A ==");
    for i in 0..a.len() {
        println!("Word {:02}: {}", i + 1, bits_to_word(&a[i]));
    }
    println!("");
    println!("== Block B ==");
    for i in 0..b.len() {
        println!("Word {:02}: {}", i + 1, bits_to_word(&b[i]));
    }
    println!("");
    println!("== Block X ==");
    for i in 0..x.len() {
        println!("Word {:02}: {}", i + 1, bits_to_word(&x[i]));
    }
}

fn restore() {
    println!("=============");
    println!("Restore Block");
    println!("=============");
    println!("");
    println!("== Enter the words of block X ==");
    let mut word_list_x = Vec::new();
    for i in 1..13 {
        word_list_x.push(input_word_bits(i));
    }
    println!("");
    println!("== Enter the words of block A or B ==");
    let mut word_list_ab = Vec::new();
    for i in 1..13 {
        word_list_ab.push(input_word_bits(i));
    }
    println!("");
    let missing = xor_wordlists(&word_list_ab, &word_list_x);
    println!("The missing block (A or B)");
    for i in 0..12 {
        println!("Word {:02}: {}", i + 1, bits_to_word(&missing[i]));
    }
}

fn main() {
    println!("==================");
    println!("Seed Phrase Raid 5");
    println!("==================");
    println!("");
    println!(" 1: Create redundancy");
    println!(" 2: Restore from redundancy");
    println!("");
    print!("What do you want to do? > ");
    io::stdout().flush().unwrap();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input == "1\n" {
                    create();
                    return;
                }
                if input == "2\n" {
                    restore();
                    return;
                }
                print!("Please choose 1 or 2 > ");
                io::stdout().flush().unwrap();
            }
            Err(error) => println!("Error while readling from command line: {}", error),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "patient wall rural drink sleep school 
        scatter twin sibling jeans panda frog believe bright 
        major bonus autumn initial regular soul weird baby ecology average";

    fn get_sample() -> Vec<String> {
        SAMPLE
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_0() {
        let words = get_sample();
        let (a, b, x) = get_a_b_x(words).unwrap();

        let b_restored = get_b_from_a_x(&a, &x).unwrap();
        assert_eq!(b_restored, b);

        let a_restored = get_b_from_a_x(&b, &x).unwrap();
        assert_eq!(a_restored, a);
    }
}
