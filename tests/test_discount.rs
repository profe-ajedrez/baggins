use baggins::{Calculator, discount, tax};


#[test]
fn test_add_discount() {
    
let mut c = baggins::DetailCalculator::new();

let err = c.add_discount_from_str(
    "10.0", 
    discount::Mode::Percentual
);
assert!(err.is_none(), "error adding percentual discount {:?}", err);

let err = c.add_tax_from_str(
    "16.0",
    tax::Stage::OverTaxable,
    tax::Mode::Percentual,
);

assert!(err.is_none(), "error adding percentual 16% tax {:?}", err);
}
