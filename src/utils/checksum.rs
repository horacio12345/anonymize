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
    
    // Extraer la parte numérica
    let number_str: String = if chars[0].is_ascii_digit() {
        // DNI: primeros 8 caracteres son dígitos
        chars[..8].iter().collect()
    } else {
        // NIE: convertir letra inicial (X=0, Y=1, Z=2) + 7 dígitos
        let first_char = chars[0].to_ascii_uppercase();
        let prefix = match first_char {
            'X' => '0',
            'Y' => '1',
            'Z' => '2',
            _ => return false, // Letra inicial inválida
        };
        
        // Construir número: prefix + 7 dígitos siguientes
        let mut num = String::new();
        num.push(prefix);
        num.push_str(&chars[1..8].iter().collect::<String>());
        num
    };
    
    // Parsear a número
    let number: u32 = match number_str.parse() {
        Ok(n) => n,
        Err(_) => return false,
    };
    
    // Calcular letra esperada
    let expected_letter = LETTERS[(number % 23) as usize];
    
    // Comparar con la letra real
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
