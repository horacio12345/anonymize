// src/utils/checksum.rs

/// IBAN (ISO 7064 Mod 97-10)
pub fn validate_iban(iban: &str) -> bool {
    let cleaned: String = iban.chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    
    if cleaned.len() < 5 {
        return false;
    }
    
    // Move first 4 chars to end
    let rearranged = format!("{}{}", &cleaned[4..], &cleaned[..4]);
    
    // Convert letters to numbers (A=10, B=11, ...)
    let numeric: String = rearranged.chars()
        .map(|c| {
            if c.is_ascii_digit() {
                c.to_string()
            } else {
                ((c as u32) - ('A' as u32) + 10).to_string()
            }
        })
        .collect();
    
    // Mod 97 check
    let remainder = numeric
        .chars()
        .fold(0u64, |acc, c| {
            (acc * 10 + c.to_digit(10).unwrap() as u64) % 97
        });
    
    remainder == 1
}

/// Spanish DNI Letter
pub fn validate_dni(dni: &str) -> bool {
    const LETTERS: &[char] = &[
        'T', 'R', 'W', 'A', 'G', 'M', 'Y', 'F', 'P', 'D',
        'X', 'B', 'N', 'J', 'Z', 'S', 'Q', 'V', 'H', 'L',
        'C', 'K', 'E'
    ];
    
    let chars: Vec<char> = dni.chars().collect();
    if chars.len() != 9 {
        return false;
    }
    
    let number: u32 = chars[..8].iter()
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    
    let expected_letter = LETTERS[(number % 23) as usize];
    chars[8].to_ascii_uppercase() == expected_letter
}

/// Luhn Algorithm
pub fn validate_luhn(number: &str) -> bool {
    let digits: Vec<u32> = number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    if digits.is_empty() {
        return false;
    }
    
    let sum: u32 = digits.iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                d
            }
        })
        .sum();
    
    sum % 10 == 0
}
