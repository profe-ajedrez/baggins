use std::str::FromStr;

use bigdecimal::BigDecimal;
use calculus::discount::{ComputedDiscount, DiscountComputer, Type};

#[test]
fn test_discount_computer() {
    discount_testable();
}

fn discount_testable() {
    let mut d = ComputedDiscount::new();

    let err = d.add_discount(BigDecimal::from_str("10.2").unwrap(), Type::Percentual);
    match err {
        Some(e) => {
            panic!("{e}")
        }
        None => {}
    }

    let err = d.add_discount_from_str("10.56", Type::AmountUnit);
    match err {
        Some(e) => {
            panic!("{e}")
        }
        None => {}
    }

    let err = d.add_discount(BigDecimal::from_str("1.5").unwrap(), Type::AmountLine);
    match err {
        Some(e) => {
            panic!("{e}")
        }
        None => {}
    }

    let res = d.compute_from_f64(100.0, 1.0);

    match res {
        Ok(disc) => {
            let expected = BigDecimal::from_str("22.26").unwrap();

            if disc != expected {
                panic!("expected {}. Got {}", expected, disc);
            }
        }
        Err(e) => {
            panic!("{e}");
        }
    }
}
