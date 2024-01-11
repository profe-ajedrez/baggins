use std::str::FromStr;

use baggins::tax::{Mode, Stage, TaxComputer, Taxer};
use bigdecimal::BigDecimal;

#[test]
fn test_tax_computer_errors() {
    let mut tax_calculator = TaxComputer::default();
    let err = tax_calculator.add_tax_from_f64(-18.0, Stage::OverTaxable, Mode::Percentual);

    match err {
        Some(_) => {}
        None => {
            panic!("{}", err.unwrap())
        }
    }
}

#[test]
fn test_tax_computer_adding_tax_f64() {
    let mut tax_calculator = TaxComputer::default();

    let err = tax_calculator.add_tax_from_f64(18.0, Stage::OverTaxable, Mode::Percentual);
    assert!(err.is_none(), "error triggered adding first f64 tax");

    let err = tax_calculator.add_tax_from_f64(10.0, Stage::OverTaxable, Mode::Percentual);
    assert!(err.is_none(), "error triggered adding second f64 tax");

    let err = tax_calculator.add_tax_from_f64(0.5, Stage::OverTaxable, Mode::AmountUnit);
    assert!(err.is_none(), "error triggered adding third f64 tax");
}

#[test]
fn test_tax_computer_calculate_over_taxable_f64() {
    let mut taxer = baggins::tax::TaxComputer::new();

    let err = taxer.add_tax_from_f64(18.0, Stage::OverTaxable, Mode::Percentual);
    assert!(err.is_none(), "error triggered adding first f64 tax");

    let err = taxer.add_tax_from_f64(10.0, Stage::OverTaxable, Mode::Percentual);
    assert!(err.is_none(), "error triggered adding second f64 tax");

    let err = taxer.add_tax_from_f64(0.5, Stage::OverTaxable, Mode::AmountUnit);
    assert!(err.is_none(), "error triggered adding third f64 tax");

    let r = taxer.tax_from_f64(24.576855, 4.0);

    match r {
        Ok(tax) => {
            let expected =
                BigDecimal::from_str("29.52607759999999814226612215861678123474121093750000")
                    .unwrap();
            assert_eq!(tax, expected);
            println!("calculated_tax: {}", tax);
        }

        Err(e) => {
            panic!("{e}")
        }
    }
}
