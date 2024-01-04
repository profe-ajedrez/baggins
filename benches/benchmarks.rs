use std::str::FromStr;

use baggins::{
    discount::{ComputedDiscount, DiscountComputer, Type},
    tax, Calculator,
};
use bigdecimal::BigDecimal;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_discount(c: &mut Criterion) {
    let vu = BigDecimal::from_str("100.0").unwrap();
    let qty = BigDecimal::from_str("1.0").unwrap();

    let mut d = ComputedDiscount::new();
    let _ = d.add_discount(BigDecimal::from_str("10.2").unwrap(), Type::Percentual);
    let _ = d.add_discount_from_str("10.56", Type::AmountUnit);
    let _ = d.add_discount(BigDecimal::from_str("1.5").unwrap(), Type::AmountLine);

    c.bench_function("bench_discount", |b| {
        b.iter(|| {
            let _ = d.compute(black_box(vu.clone()), black_box(qty.clone()));
        });
    });
}

fn bench_compute(c: &mut Criterion) {
    let vu = BigDecimal::from_str("100.0").unwrap();
    let qty = BigDecimal::from_str("1.0").unwrap();

    let mut cl = baggins::DetailCalculator::new();
    let _ = cl.add_discount(BigDecimal::from_str("10.2").unwrap(), Type::Percentual);
    let _ = cl.add_discount_from_str("10.56", Type::AmountUnit);
    let _ = cl.add_discount(BigDecimal::from_str("1.5").unwrap(), Type::AmountLine);

    let _ = cl.add_tax_from_str(
        "16.0",
        tax::tax_stage::Stage::OverTaxable,
        tax::Type::Percentual,
    );

    let _ = cl.add_tax_from_str(
        "1.0",
        tax::tax_stage::Stage::OverTaxable,
        tax::Type::AmountUnit,
    );

    c.bench_function("bench_discount", |b| {
        b.iter(|| {
            let _ = cl.compute(black_box(vu.clone()), black_box(qty.clone()), black_box(16));
        });
    });
}

criterion_group!(benches, bench_discount, bench_compute);
criterion_main!(benches);
