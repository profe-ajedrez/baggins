// Copyright 2023 Andrés Reyes El Programador Pobre.
//
//! baggins
//!
//! `baggins` provides a series of utilities to easily and efficiently calculate sales operations.
//! Due to the nature of monetary calculations, the Bigdecimal crate is used as a backend.
//!
//! The focus is on ease of use and learning Rust, so there are many opportunities for improvement.
//!
//! `baggins` provee una serie de utilidades para calcular facil y eficientemente totales
//! y subtotales de lineas de detalle de procesos de venta.
//! El foco está en la facilidad de uso y en aprender Rust, por lo que hay muchas oportunidades de mejora.
//!
//!
use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use discount::Discounter;
use serde::Serialize;
use std::{fmt, str::FromStr};
use tax::Taxer;

pub mod discount;
pub mod tax;

/// handy utility to get 100.0 as BigDecimal
pub fn hundred() -> BigDecimal {
    BigDecimal::from_str("100.0").unwrap()
}

/// handy utility to get 1.0 as BigDecimal
pub fn one() -> BigDecimal {
    BigDecimal::from_str("1.0").unwrap()
}

/// handy utility to get -1.0 as BigDecimal
pub fn inverse() -> BigDecimal {
    BigDecimal::from_str("-1.0").unwrap()
}

/// handy utility to get 0.0 as BigDecimal. Just a wrapper for zero()
pub fn zero() -> BigDecimal {
    BigDecimal::zero()
}

#[derive(Debug)]
/// The error type for baggins operations - El tipo de error para operaciones de baggins
///
/// Everything in life can end in a huge mistake, and the operations that baggins performs
/// They are not the exception. We have tried to prepare the most common errors.
///
/// Todo en la vida puede terminar en un enorme error, y las operaciones que baggins realiza
/// no son la excepción. Hemos tratado de preparar los errores mas comunes.
///
/// # Example
///
/// ```
///  use baggins::discount::Mode;
///  use baggins::DetailCalculator;
///  use crate::baggins::Calculator;
///
///  let mut c = DetailCalculator::new();
///
///  c.add_discount_from_str("22.74", Mode::Percentual);
///
///  let r = c.compute_from_str("120.34", "-10", None);
///
///  match r {
///      Ok(calculation) => {
///          println!("this branch will not be executed");
///      }
///      Err(err) => {
///          println!("this will print a bagginsError::NegativeQty {}", err);
///      }
///  }
/// ```
///
pub enum BagginsError<S: Into<String>> {

    /// Error due to pass a negative quantity
    NegativeQty(S),

    /// Error for not being able to convert a value to [BigDecimal]
    InvalidDecimalValue(S),

    /// Any other unspecified error
    Other(S),
}

impl<S: Into<String> + Clone> fmt::Display for BagginsError<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BagginsError::NegativeQty(msg) => {
                write!(f, "Negative quantity value Error: {}", msg.clone().into())
            }
            BagginsError::InvalidDecimalValue(msg) => {
                write!(f, " Invalid decimal value {}", msg.clone().into())
            }
            BagginsError::Other(msg) => write!(f, " Error {}", msg.clone().into()),
        }
    }
}

#[derive(Debug, Serialize)]
/// will contain the result of the computing of the specified subtotal
pub struct CalculationWithDiscount {
    /// stores the unit value multiplied by the quantity minus the discount
    pub net: BigDecimal,
    /// stores the net plus taxes
    pub brute: BigDecimal,
    /// stores the cumulated tax calculated over net
    pub tax: BigDecimal,
    /// stores the cumulated discount value
    pub discount_value: BigDecimal,
    /// stores the cumulated discount value
    pub discount_brute_value: BigDecimal,
    /// stores the total discount applied as a percentage
    pub total_discount_percent: BigDecimal,
    /// stores the unit value with discounts applied
    pub unit_value: BigDecimal,
}

impl CalculationWithDiscount {
    /// Creates a new [`CalculationWithDiscount`].
    pub fn new(
        net: BigDecimal,
        brute: BigDecimal,
        tax: BigDecimal,
        discount_value: BigDecimal,
        discount_brute_value: BigDecimal,
        total_discount_percent: BigDecimal,
        unit_value: BigDecimal,
    ) -> Self {
        Self {
            net,
            brute,
            tax,
            discount_value,
            discount_brute_value,
            total_discount_percent,
            unit_value,
        }
    }
}

impl Default for CalculationWithDiscount {
    fn default() -> Self {
        Self {
            net: zero(),
            brute: zero(),
            tax: zero(),
            discount_value: zero(),
            discount_brute_value: zero(),
            total_discount_percent: zero(),
            unit_value: zero(),
        }
    }
}

impl fmt::Display for CalculationWithDiscount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "net {}, brute {}, tax {}, discount value {}, discount brute value {}, total discount percent {}, unit_value {} )",
            self.net,
            self.brute,
            self.tax,
            self.discount_value,
            self.discount_brute_value,
            self.total_discount_percent,
            self.unit_value,
        )
    }
}

#[derive(Debug, Serialize)]
/// will contain the result of the computing of the specified subtotal without discounts
pub struct CalculationWithoutDiscount {
    /// stores the unit value multiplied by the quantity
    pub net: BigDecimal,
    /// stores the net plus taxes
    pub brute: BigDecimal,
    /// stores the cumulated tax calculated over net
    pub tax: BigDecimal,
    /// stores the used unit value
    pub unit_value: BigDecimal,
}

impl CalculationWithoutDiscount {
    /// Creates a new [`CalculationWithoutDiscount`].
    pub fn new(
        net: BigDecimal,
        brute: BigDecimal,
        tax: BigDecimal,
        unit_value: BigDecimal,
    ) -> Self {
        Self {
            net,
            brute,
            tax,
            unit_value,
        }
    }
}

impl Default for CalculationWithoutDiscount {
    fn default() -> Self {
        Self {
            net: zero(),
            brute: zero(),
            tax: zero(),
            unit_value: zero(),
        }
    }
}

impl fmt::Display for CalculationWithoutDiscount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "net {}, brute {}, tax {}, unit_value {})",
            self.net, self.brute, self.tax, self.unit_value,
        )
    }
}

#[derive(Debug, Serialize, Default)]
pub struct Calculation {
    without_discount_values: CalculationWithoutDiscount,
    with_discount_values: CalculationWithDiscount,
}


impl Calculation {
    pub fn new(
        without_discount_values: CalculationWithoutDiscount,
        with_discount_values: CalculationWithDiscount,
    ) -> Self {
        Self {
            without_discount_values,
            with_discount_values,
        }
    }
}

impl fmt::Display for Calculation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "without_discount_valueset {}, with_discount_values {}",
            self.without_discount_values, self.with_discount_values,
        )
    }
}

// A thing able to calculate sales values
pub trait Calculator {
    /// adds a [BigDecimal] discount value of the specified [discount::Mode] to [Calculator].
    fn add_discount(
        &mut self,
        discount: BigDecimal,
        discount_mode: discount::Mode,
    ) -> Option<discount::DiscountError<String>>;

    /// adds a [f64] discount value of the specified [discount::Mode]
    /// to [Calculator] so expect some precision loss
    fn add_discount_from_f64(
        &mut self,
        discount: f64,
        discount_mode: discount::Mode,
    ) -> Option<discount::DiscountError<String>>;

    /// adds an [`Into<String>`] discount value of the specified [discount::Mode]
    /// to [`Calculator`]
    fn add_discount_from_str<S: Into<String>>(
        &mut self,
        discount: S,
        discount_mode: discount::Mode,
    ) -> Option<discount::DiscountError<String>>;

    /// adds a tax to the specified [tax::Stage] in [Calculator] from a [f64] value
    /// of the specified [tax::Mode] so expect some precision loss
    fn add_tax_from_f64(
        &mut self,
        tax: f64,
        stage: tax::Stage,
        tax_mode: tax::Mode,
    ) -> Option<tax::TaxError<String>>;

    /// adds a tax to the specified [tax::Stage] in [Calculator] from a [BigDecimal]
    fn add_tax(
        &mut self,
        tax: BigDecimal,
        stage: tax::Stage,
        tax_mode: tax::Mode,
    ) -> Option<tax::TaxError<String>>;

    /// adds a tax to the specified [tax::Stage] in [Calculator] from a [String]
    fn add_tax_from_str<S: Into<String>>(
        &mut self,
        tax: S,
        stage: tax::Stage,
        tax_mode: tax::Mode,
    ) -> Option<tax::TaxError<String>>;

    /// calculates and produces a [Calculation] from a [BigDecimal] brute value
    /// and a quantity of the same type
    fn compute_from_brute(
        &mut self,
        brute: BigDecimal,
        qty: BigDecimal,
        max_discount_allowed: Option<BigDecimal>,
    ) -> Result<Calculation, BagginsError<String>>;

    /// calculates and produces a [Calculation] from a [f64] brute subtotal value
    /// and a quantity of the same type. Use of [f64] may cause precission loss
    fn compute_from_brute_f64(
        &mut self,
        brute: f64,
        qty: f64,
        max_discount_allowed: Option<f64>,
    ) -> Result<Calculation, BagginsError<String>>;

    /// calculates and produces a [Calculation] from a [String] brute value
    /// and a quantity of the same type
    fn compute_from_brute_str<S: Into<String>>(
        &mut self,
        brute: S,
        qty: S,
        max_discount_allowed: Option<S>,
    ) -> Result<Calculation, BagginsError<String>>;

    /// calculates and produces a [Calculation] from a [String] unit value
    /// and a quantity of the same type
    fn compute_from_str<S: Into<String>>(
        &mut self,
        unit_value: S,
        qty: S,
        max_discount_allowed: Option<S>,
    ) -> Result<Calculation, BagginsError<String>>;

    /// calculates and produces a [Calculation] from a [f64] unit value
    /// and a quantity of the same type. Use of [f64] may cause precission loss
    fn compute_from_f64(
        &mut self,
        unit_value: f64,
        qty: f64,
        max_discount_allowed: Option<f64>,
    ) -> Result<Calculation, BagginsError<String>>;

    /// calculates and produces a [Calculation] from a [BigDecimal] unit value
    /// and a quantity of the same type
    fn compute(
        &mut self,
        unit_value: BigDecimal,
        qty: BigDecimal,
        max_discount_allowed: Option<BigDecimal>,
    ) -> Result<Calculation, BagginsError<String>>;

    /// an utility to calculate a tax directly
    ///
    /// # Params
    ///
    /// taxable [BigDecimal] value to tax
    ///
    /// qty     [BigDecimal] quantity being sold (some tax needs to know this value to be calculated)
    ///
    /// value   [BigDecimal] percent or amount to apply as tax
    ///
    /// mode    [tax::Mode]  wheter the tax is percentual, amount by unit or amount by line
    ///
    fn line_tax(
        &mut self,
        taxable: BigDecimal,
        qty: BigDecimal,
        value: BigDecimal,
        mode: tax::Mode,
    ) -> Result<BigDecimal, tax::TaxError<String>>;

    /// an utility to calculate a tax directly using [String]s as entry.
    /// Converts values to BigDecimal.
    ///
    /// # Params
    ///
    /// taxable [String] value to tax
    ///
    /// qty     [String] quantity being sold (some tax needs to know this value to be calculated)
    ///
    /// value   [String] percent or amount to apply as tax
    ///
    /// mode    [tax::Mode]  wheter the tax is percentual, amount by unit or amount by line
    ///
    fn line_tax_from_str<S: Into<String>>(
        &mut self,
        taxable: S,
        qty: S,
        value: S,
        mode: tax::Mode,
    ) -> Result<BigDecimal, tax::TaxError<String>>;

    /// an utility to calculate a tax directly using [f64]s as entry. Some precission could be loss.
    /// Converts values to BigDecimal.
    ///
    /// # Params
    ///
    /// taxable [f64] value to tax
    ///
    /// qty     [f64] quantity being sold (some tax needs to know this value to be calculated)
    ///
    /// value   [f64] percent or amount to apply as tax
    ///
    /// mode    [tax::Mode]  wheter the tax is percentual, amount by unit or amount by line
    ///
    fn line_tax_from_f64(
        &mut self,
        taxable: f64,
        qty: f64,
        value: f64,
        mode: tax::Mode,
    ) -> Result<BigDecimal, tax::TaxError<String>>;
}

pub struct DetailCalculator {
    tax_handler: tax::TaxComputer,
    discount_handler: discount::DiscountComputer,
}

impl DetailCalculator {
    /// Creates a new [`DetailCalculator`].
    pub fn new() -> Self {
        Self {
            tax_handler: tax::TaxComputer::default(),
            discount_handler: discount::DiscountComputer::default(),
        }
    }
}

impl Default for DetailCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl Calculator for DetailCalculator {
    fn add_discount(
        &mut self,
        discount: BigDecimal,
        discount_mode: discount::Mode,
    ) -> Option<discount::DiscountError<String>> {
        self.discount_handler.add_discount(discount, discount_mode)
    }

    fn add_discount_from_f64(
        &mut self,
        discount: f64,
        discount_mode: discount::Mode,
    ) -> Option<discount::DiscountError<String>> {
        self.discount_handler
            .add_discount_from_f64(discount, discount_mode)
    }

    fn add_discount_from_str<S: Into<String>>(
        &mut self,
        discount: S,
        discount_mode: discount::Mode,
    ) -> Option<discount::DiscountError<String>> {
        self.discount_handler
            .add_discount_from_str(discount, discount_mode)
    }

    fn add_tax_from_f64(
        &mut self,
        tax: f64,
        stage: tax::Stage,
        tax_mode: tax::Mode,
    ) -> Option<tax::TaxError<String>> {
        self.tax_handler.add_tax_from_f64(tax, stage, tax_mode)
    }

    fn add_tax(
        &mut self,
        tax: BigDecimal,
        stage: tax::Stage,
        tax_mode: tax::Mode,
    ) -> Option<tax::TaxError<String>> {
        self.tax_handler.add_tax(tax, stage, tax_mode)
    }

    fn add_tax_from_str<S: Into<String>>(
        &mut self,
        tax: S,
        stage: tax::Stage,
        tax_mode: tax::Mode,
    ) -> Option<tax::TaxError<String>> {
        self.tax_handler.add_tax_from_str(tax, stage, tax_mode)
    }

    fn compute_from_brute(
        &mut self,
        brute: BigDecimal,
        qty: BigDecimal,
        max_discount_allowed: Option<BigDecimal>,
    ) -> Result<Calculation, BagginsError<String>> {
        match self.tax_handler.un_tax(brute.clone(), qty.clone()) {
            Ok(un_taxed) => match self
                .discount_handler
                .un_discount(un_taxed.clone(), qty.clone())
            {
                Ok(un_discounted) => self.compute(un_discounted.0, qty, max_discount_allowed),
                Err(err) => Err(BagginsError::Other(format!(
                    "undiscounting un_taxed {} {}",
                    un_taxed, err
                ))),
            },
            Err(err) => Err(BagginsError::Other(format!(
                "untaxing brute {} {}",
                brute, err
            ))),
        }
    }

    fn compute_from_brute_f64(
        &mut self,
        brute: f64,
        qty: f64,
        max_discount_allowed: Option<f64>,
    ) -> Result<Calculation, BagginsError<String>> {
        
        let max_discount_allowed: Option<BigDecimal> = BigDecimal::from_f64(max_discount_allowed.unwrap_or(100.0f64));

        self.compute_from_brute(
            BigDecimal::from_f64(brute).unwrap_or(inverse()),
            BigDecimal::from_f64(qty).unwrap_or(inverse()),
            max_discount_allowed,
        )
    }

    fn compute_from_brute_str<S: Into<String>>(
        &mut self,
        brute: S,
        qty: S,
        max_discount_allowed: Option<S>,
    ) -> Result<Calculation, BagginsError<String>> {
        let brute = brute.into();
        let qty = qty.into();

        match BigDecimal::from_str(&brute) {
            Ok(brute) => match BigDecimal::from_str(&qty) {
                Ok(qty) => match max_discount_allowed {
                    Some(max_discount_allowed) => {
                        let max_discount_allowed = max_discount_allowed.into();

                        match BigDecimal::from_str(&max_discount_allowed) {
                            Ok(max_discount_allowed) => {
                                self.compute(brute, qty, Some(max_discount_allowed))
                            }
                            Err(err) => Err(BagginsError::InvalidDecimalValue(format!(
                                "parsing max_discount_allowed: <S: Into<String>> {} {}",
                                max_discount_allowed, err,
                            ))),
                        }
                    }
                    None => self.compute(brute, qty, None),
                },
                Err(err) => Err(BagginsError::InvalidDecimalValue(format!(
                    "parsing qty: <S: Into<String>> {} {}",
                    qty, err,
                ))),
            },
            Err(err) => Err(BagginsError::InvalidDecimalValue(format!(
                "parsing unit_value: <S: Into<String>> {} {}",
                brute, err,
            ))),
        }
    }

    fn compute_from_str<S: Into<String>>(
        &mut self,
        unit_value: S,
        qty: S,
        max_discount_allowed: Option<S>,
    ) -> Result<Calculation, BagginsError<String>> {
        let unit_value = unit_value.into();
        let qty = qty.into();

        match BigDecimal::from_str(&unit_value) {
            Ok(unit_value) => match BigDecimal::from_str(&qty) {
                Ok(qty) => match max_discount_allowed {
                    Some(max_discount_allowed) => {
                        let max_discount_allowed = max_discount_allowed.into();

                        match BigDecimal::from_str(&max_discount_allowed) {
                            Ok(max_discount_allowed) => {
                                self.compute(unit_value, qty, Some(max_discount_allowed))
                            }
                            Err(err) => Err(BagginsError::InvalidDecimalValue(format!(
                                "parsing max_discount_allowed: <S: Into<String>> {} {}",
                                max_discount_allowed, err,
                            ))),
                        }
                    }
                    None => self.compute(unit_value, qty, None),
                },
                Err(err) => Err(BagginsError::InvalidDecimalValue(format!(
                    "parsing qty: <S: Into<String>> {} {}",
                    qty, err,
                ))),
            },
            Err(err) => Err(BagginsError::InvalidDecimalValue(format!(
                "parsing unit_value: <S: Into<String>> {} {}",
                unit_value, err,
            ))),
        }
    }

    fn compute_from_f64(
        &mut self,
        unit_value: f64,
        qty: f64,
        max_discount_allowed: Option<f64>,
    ) -> Result<Calculation, BagginsError<String>> {
        let max_discount_allowed: Option<BigDecimal> = BigDecimal::from_f64(max_discount_allowed.unwrap_or(100.0f64));

        self.compute(
            BigDecimal::from_f64(unit_value).unwrap_or(inverse()),
            BigDecimal::from_f64(qty).unwrap_or(inverse()),
            max_discount_allowed,
        )
    }

    fn compute(
        &mut self,
        unit_value: BigDecimal,
        qty: BigDecimal,
        max_discount_allowed: Option<BigDecimal>,
    ) -> Result<Calculation, BagginsError<String>> {
        match self
            .discount_handler
            .compute(unit_value.clone(), qty.clone(), max_discount_allowed)
        {
            Ok(discount) => {
                let net = &unit_value * &qty - &discount.0;
                let discounted_uv = &net / &qty;

                match self.tax_handler.tax(discounted_uv.clone(), qty.clone()) {
                    Ok(tax) => match self.tax_handler.tax(unit_value.clone(), qty.clone()) {
                        Ok(tax_without_discount) => {
                            let net_without_discount = &unit_value * &qty;
                            let brute_without_discount =
                                &net_without_discount + &tax_without_discount;
                            let brute = &net + &tax;

                            let calc = Calculation {
                                without_discount_values: CalculationWithoutDiscount {
                                    brute: brute_without_discount.clone(),
                                    unit_value: &net_without_discount / &qty,
                                    net: net_without_discount,
                                    tax: tax_without_discount,
                                },
                                with_discount_values: CalculationWithDiscount {
                                    discount_brute_value: &brute - &brute_without_discount,
                                    brute,
                                    unit_value: &net / &qty,
                                    net,
                                    tax,
                                    discount_value: discount.0,
                                    total_discount_percent: discount.1,
                                },
                            };

                            Ok(calc)
                        }
                        Err(err) => Err(BagginsError::Other(format!(
                            "calculating taxes {}",
                            err
                        ))),
                    },
                    Err(err) => Err(BagginsError::Other(format!(
                        "calculating taxes {}",
                        err
                    ))),
                }
            }
            Err(err) => Err(BagginsError::Other(format!(
                "calculating discount {}",
                err
            ))),
        }
    }

    fn line_tax(
        &mut self,
        taxable: BigDecimal,
        qty: BigDecimal,
        value: BigDecimal,
        mode: tax::Mode,
    ) -> Result<BigDecimal, tax::TaxError<String>> {
        self.tax_handler.line_tax(taxable, qty, value, mode)
    }

    fn line_tax_from_str<S: Into<String>>(
        &mut self,
        taxable: S,
        qty: S,
        value: S,
        mode: tax::Mode,
    ) -> Result<BigDecimal, tax::TaxError<String>> {
        self.tax_handler
            .line_tax_from_str(taxable, qty, value, mode)
    }

    fn line_tax_from_f64(
        &mut self,
        taxable: f64,
        qty: f64,
        value: f64,
        mode: tax::Mode,
    ) -> Result<BigDecimal, tax::TaxError<String>> {
        self.tax_handler
            .line_tax_from_f64(taxable, qty, value, mode)
    }
}
