use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Please input your Palindrome: ");
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_uppercase() == "END" {
            break;
        }
        if is_palindrome(input.trim().to_string()) {
            println!("{} is palindrome", input);
        } else {
            println!("{} is not palindrome", input);
        }
    }
}

fn is_palindrome(palindrome: String) -> bool {
    let palindrome_length = palindrome.len();
    let is_even = palindrome_length % 2 == 0;
    if is_even {
        let front_palindrom = &palindrome[..palindrome_length / 2];
        let back_palindrom = &palindrome[(palindrome_length / 2)..]
            .chars()
            .rev()
            .collect::<String>();
        if front_palindrom == back_palindrom {
            return true;
        } else {
            return false;
        }
    } else {
        let front_palindrom = &palindrome[..palindrome_length / 2];
        let back_palindrom = &palindrome[(palindrome_length / 2) + 1..]
            .chars()
            .rev()
            .collect::<String>();
        if front_palindrom == back_palindrom {
            return true;
        } else {
            return false;
        }
    }
}
