use std::io;

fn is_palindrome(phrase: &str) -> bool {
    if phrase.len() == 0 || phrase.len() == 1  { return false }

    let phrase = phrase.trim();
    let phrase: Vec<char> = phrase.chars().collect();

    let mut first_idx = 0;
    let mut last_idx = phrase.len() - 1;

    while first_idx < last_idx {
        if !phrase[first_idx].is_alphabetic() { first_idx += 1; continue }
        if !phrase[last_idx].is_alphabetic() { last_idx -= 1; continue }

         if phrase[first_idx].to_ascii_lowercase() != phrase[last_idx].to_ascii_lowercase() {
            return false;
        }

        first_idx += 1;
        last_idx -= 1;
    }

    return true;
}

fn main() {

    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Error reading input");
    println!("length: {}",input_text.trim().len());
    println!("palindrome {} is {}",input_text,is_palindrome(&input_text));


}



// fn check_palindrome(text: &str) -> bool {
//   if text.trim().len() = 0 { return false; }

//   A N N A 2 = 0
//   C I V I C 2 = 1
//   0 1 2 3 4

//   A B B D C D B B A
//   0 1 2 3 4 5 6 7 8
// }