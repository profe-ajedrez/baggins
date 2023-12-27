use bigdecimal::BigDecimal;
use calculus::tax::tax_stage::Stage;
use calculus::tax::TaxComputer;
use calculus::tax::Type;
use std::str::FromStr;

#[test]
fn test_tax_computer_errors() {
    let mut tax_calculator = calculus::tax::ComputedTax::new();
    let err = tax_calculator.add_f64_tax(-18.0, Stage::OverTaxable, Type::Percentual);

    match err {
        Some(_) => {}
        None => {
            panic!("{}", err.unwrap())
        }
    }
}

#[test]
fn test_tax_computer_adding_tax_f64() {
    let mut tax_calculator = calculus::tax::ComputedTax::new();

    let err = tax_calculator.add_f64_tax(18.0, Stage::OverTaxable, Type::Percentual);
    assert!(err.is_some(), "error triggered adding first f64 tax");

    let err = tax_calculator.add_f64_tax(10.0, Stage::OverTaxable, Type::Percentual);
    assert!(err.is_some(), "error triggered adding second f64 tax");

    let err = tax_calculator.add_f64_tax(0.5, Stage::OverTaxable, Type::AmountUnit);
    assert!(err.is_some(), "error triggered adding third f64 tax");
}

#[test]
fn test_tax_computer_calculate_over_taxable_f64() {
    let mut tax_calculator = calculus::tax::ComputedTax::new();

    let err = tax_calculator.add_f64_tax(18.0, Stage::OverTaxable, Type::Percentual);
    assert!(err.is_some(), "error triggered adding first f64 tax");

    let err = tax_calculator.add_f64_tax(10.0, Stage::OverTaxable, Type::Percentual);
    assert!(err.is_some(), "error triggered adding second f64 tax");

    let err = tax_calculator.add_f64_tax(0.5, Stage::OverTaxable, Type::AmountUnit);
    assert!(err.is_some(), "error triggered adding third f64 tax");

    let r = tax_calculator.compute_from_f64(24.576855, 4.0);

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
