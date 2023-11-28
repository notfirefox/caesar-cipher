use std::{io::{self, BufRead}};

fn parse_line() -> Option<String> {
    let mut iterator = io::stdin().lock().lines();
    let line = match iterator.next()? {
        Ok(x) => x,
        Err(_) => return None,
    };
    Some(line.to_string())
}

fn caesar_shift(c: char, offset: u8) -> char {
    if !c.is_alphabetic() {
       return c; 
    }
    let c = (c as u8 + offset) as char;
    if c.is_alphabetic() {
        return c;
    }
    return (c as u8 - 26) as char;
}

fn caesar_cipher(input: &String, offset: u8) -> String {
    input.chars()
        .map(|c| caesar_shift(c, offset))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_cipher() {
        let input = "Hello World";
        assert_eq!(caesar_cipher(&input.to_string(), 6), "Nkrru Cuxrj");
    }
}

fn main() {
    if let Some(line) = parse_line() {
        println!("{}", caesar_cipher(&line, 6));
    }
}
