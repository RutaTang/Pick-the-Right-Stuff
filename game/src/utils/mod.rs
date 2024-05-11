pub mod shuffle;
pub mod tcp;


pub fn to_ordinal(num: u32) -> String {
    let suffix = match (num % 10, num % 100) {
        (1, 11) => "th",
        (1, _) => "st",
        (2, 12) => "th",
        (2, _) => "nd",
        (3, 13) => "th",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{}{}", num, suffix)
}

pub fn to_number(ordinal: &str) -> Option<u32> {
    let num_str = ordinal.trim_end_matches(|c: char| !c.is_digit(10));
    match num_str.parse::<u32>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}