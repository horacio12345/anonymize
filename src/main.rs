// src/main.rs

use anonymize::{
    Anonymizer, EmailDetector, PhoneDetector, DniDetector, IbanDetector,
    CreditCardDetector, SsnDetector, ProjectCodeDetector, ContractNumberDetector,
    WorkOrderDetector, PurchaseOrderDetector, SerialNumberDetector,
    CostCenterDetector, Result,
};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut engine = Anonymizer::new();
    
    // Personal data detectors
    engine.add_detector(Box::new(EmailDetector::new()));
    engine.add_detector(Box::new(PhoneDetector::new()));
    engine.add_detector(Box::new(DniDetector::new()));
    engine.add_detector(Box::new(IbanDetector::new()));
    engine.add_detector(Box::new(CreditCardDetector::new()));
    engine.add_detector(Box::new(SsnDetector::new()));
    
    // Corporate/Industrial detectors
    engine.add_detector(Box::new(ProjectCodeDetector::new()));
    engine.add_detector(Box::new(ContractNumberDetector::new()));
    engine.add_detector(Box::new(WorkOrderDetector::new()));
    engine.add_detector(Box::new(PurchaseOrderDetector::new()));
    engine.add_detector(Box::new(SerialNumberDetector::new()));
    engine.add_detector(Box::new(CostCenterDetector::new()));

    let output = engine.anonymize(&input)?;

    println!("--- ANONYMIZED TEXT ---");
    println!("{}", output.text);
    println!("\n--- AUDIT REPORT (JSON) ---");
    println!("{}", serde_json::to_string_pretty(&output.report).unwrap());

    Ok(())
}