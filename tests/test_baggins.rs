use std::str::FromStr;

use baggins::{DetailCalculator, discount, Calculator, tax};
use bigdecimal::BigDecimal;




#[test]
fn test_baggins_compute() {
    let mut c = DetailCalculator::new();

    let err = c.add_discount_from_str("10.0", discount::Mode::Percentual);
    assert!(err.is_none(), "error adding percentual discount {:?}", err);

    let err = c.add_discount_from_str("1.0", discount::Mode::AmountUnit);
    assert!(err.is_none(), "error adding amount unit discount {:?}", err);

    // let err: Option<discount::DiscountError> = c.add_str_discount("2.0", discount::Mode::AmountLine);
    // assert!(err.is_none(), "error adding amount line discount");

    let err = c.add_tax_from_str(
        "16.0",
        tax::Stage::OverTaxable,
        tax::Mode::Percentual,
    );
    assert!(err.is_none(), "error adding percentual 16% tax {:?}", err);

    let err = c.add_tax_from_str(
        "1.0",
        tax::Stage::OverTaxable,
        tax::Mode::AmountUnit,
    );
    assert!(
        err.is_none(),
        "error adding percentual 1 amount unit tax {:?}",
        err
    );

    let r = c.compute(
        BigDecimal::from_str("100.0").unwrap(),
        BigDecimal::from_str("2.0").unwrap(),
        None,
    );

    match r {
        Ok(calc) => {
            println!("calc: {}", calc);
        }
        Err(e) => {
            panic!("{e}")
        }
    }
}