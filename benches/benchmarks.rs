use std::str::FromStr;

use baggins::{
    discount::{DiscountComputer, Discounter, Mode, self},
    tax::{Mode as TaxMode, Stage, Taxer, self}, DetailCalculator, Calculator,
};
use bigdecimal::BigDecimal;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_discount(c: &mut Criterion) {
    let vu = BigDecimal::from_str("100.0").unwrap();
    let qty = BigDecimal::from_str("1.0").unwrap();

    let mut d = DiscountComputer::new();
    let _ = d.add_discount(BigDecimal::from_str("10.2").unwrap(), Mode::Percentual);
    let _ = d.add_discount_from_str("10.56", Mode::AmountUnit);
    let _ = d.add_discount(BigDecimal::from_str("1.5").unwrap(), Mode::AmountLine);

    c.bench_function("bench_discount", |b| {
        b.iter(|| {
            let _ = d.compute(black_box(vu.clone()), black_box(qty.clone()), None);
        });
    });
}

fn bench_taxes(c: &mut Criterion) {
    let vu = BigDecimal::from_str("100.0").unwrap();
    let qty = BigDecimal::from_str("1.0").unwrap();

    let mut taxer = baggins::tax::TaxComputer::new();
    let _ = taxer.add_tax_from_f64(18.0, Stage::OverTaxable, TaxMode::Percentual);
    let _ = taxer.add_tax_from_f64(10.0, Stage::OverTaxable, TaxMode::Percentual);
    let _ = taxer.add_tax_from_f64(0.5, Stage::OverTaxable, TaxMode::AmountUnit);

    c.bench_function("bench_taxes", |b| {
        b.iter(|| {
            let _ = taxer.tax(black_box(vu.clone()), black_box(qty.clone()));
        });
    });
}


fn bench_baggins(c: &mut Criterion) {
    let mut cl = DetailCalculator::new();

    let _ = cl.add_discount_from_str("10.0", discount::Mode::Percentual);

    let _ = cl.add_discount_from_str("1.0", discount::Mode::AmountUnit);

    // let err: Option<discount::DiscountError> = c.add_str_discount("2.0", discount::Mode::AmountLine);
    // assert!(err.is_none(), "error adding amount line discount");

    let _ = cl.add_tax_from_str(
        "16.0",
        tax::Stage::OverTaxable,
        tax::Mode::Percentual,
    );    

    let _ = cl.add_tax_from_str(
        "1.0",
        tax::Stage::OverTaxable,
        tax::Mode::AmountUnit,
    );


    c.bench_function("bench_baggins", |b| {
        b.iter(|| {
            let _ = cl.compute(
                BigDecimal::from_str("100.0").unwrap(),
                BigDecimal::from_str("2.0").unwrap(),
                None,
            );
        });
    });
}

// fn bench_compute(c: &mut Criterion) {
//     let vu = BigDecimal::from_str("100.0").unwrap();
//     let qty = BigDecimal::from_str("1.0").unwrap();

//     let mut cl = baggins::DetailCalculator::new();
//     let _ = cl.add_discount(BigDecimal::from_str("10.2").unwrap(), Mode::Percentual);
//     let _ = cl.add_discount_from_str("10.56", Mode::AmountUnit);
//     let _ = cl.add_discount(BigDecimal::from_str("1.5").unwrap(), Mode::AmountLine);

//     let _ = cl.add_tax_from_str(
//         "16.0",
//         tax::tax_stage::Stage::OverTaxable,
//         tax::Type::Percentual,
//     );

//     let _ = cl.add_tax_from_str(
//         "1.0",
//         tax::tax_stage::Stage::OverTaxable,
//         tax::Type::AmountUnit,
//     );

//     c.bench_function("bench_discount", |b| {
//         b.iter(|| {
//             let _ = cl.compute(black_box(vu.clone()), black_box(qty.clone()), black_box(16));
//         });
//     });
// }

criterion_group!(benches, bench_discount, bench_taxes, bench_baggins);
criterion_main!(benches);
