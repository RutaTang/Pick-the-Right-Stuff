pub mod shuffle;


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