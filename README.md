# baggins

Utilities to Perform Sales Totals and Subtotals Calculation Operations.

## Use

baggins exposes a calculator to obtain sales subtotals, into which taxes and discounts can be entered. 

```rust

let mut c = baggins::DetailCalculator::new();

let err = c.add_discount_from_str(
    "10.0", 
    discount::Type::Percentual
);
assert!(err.is_none(), "error adding percentual discount {:?}", err);

let err = c.add_tax_from_str(
    "16.0",
    tax::tax_stage::Stage::OverTaxable,
    tax::Type::Percentual,
);

assert!(err.is_none(), "error adding percentual 16% tax {:?}", err);
```

Once taxes and discounts have been entered, the `compute` method can be called to obtain the results

```rust
let r = c.compute(
    BigDecimal::from_str("100.0").unwrap(),
    BigDecimal::from_str("2.0").unwrap(),
    16,
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


