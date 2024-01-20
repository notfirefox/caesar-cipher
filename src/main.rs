use std::{io::{self, BufRead}};

fn parse_line() -> Option<String> {
    io::stdin().lock().lines().next()?.ok()
}

fn caesar_shift(c: char, offset: u8) -> char {
    if !c.is_alphabetic() {
       return c; 
    }
    let c = (c as u8 + (offset % 26)) as char;
    if c.is_alphabetic() {
        return c;
    }
    (c as u8 - 26) as char
}

fn caesar_cipher(input: &str, offset: u8) -> String {
    input.chars()
        .map(|c| caesar_shift(c, offset))
        .collect()
}

fn main() {
    if let Some(line) = parse_line() {
        println!("{}", caesar_cipher(&line, 6));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_cipher() {
        let input = "Hello World";
        assert_eq!(caesar_cipher(input, 6), "Nkrru Cuxrj");
    }
}
