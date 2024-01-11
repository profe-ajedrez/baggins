# baggins

Utilities to Perform Sales Totals and Subtotals Calculation Operations.

## Use

baggins exposes a calculator to obtain sales subtotals, into which taxes and discounts can be entered. 

```rust
use baggins::{Calculator, discount, tax};

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
```

Once taxes and discounts have been entered, the `compute` method can be called to obtain the results

```rust
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
```


