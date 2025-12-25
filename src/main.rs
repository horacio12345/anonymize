// src/main.rs

use anonymize::{Anonymizer, EmailDetector, PhoneDetector, DniDetector, Result};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut engine = Anonymizer::new();
    engine.add_detector(Box::new(EmailDetector::new()));
    engine.add_detector(Box::new(PhoneDetector::new())); 
    engine.add_detector(Box::new(DniDetector::new()));

    let output = engine.anonymize(&input)?;

    println!("--- ANONYMIZED TEXT ---");
    println!("{}", output.text);
    println!("\n--- AUDIT REPORT (JSON) ---");
    println!("{}", serde_json::to_string_pretty(&output.report).unwrap());

    Ok(())
}
