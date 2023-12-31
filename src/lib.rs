// Copyright 2023 Andrés Reyes El Programador Pobre.
//
//! baggins
//!
//! `baggins` provides a series of utilities to easily and efficiently calculate totals
//! and subtotals for sales.
//! Due to the nature of monetary calculations, the Bigdecimal crate is used as a backend.
//!
//! The focus is on ease of use and learning Rust, so there are many opportunities for improvement.
//!
//! `baggins` provee una serie de utilidades para calcular facil y eficientemente totales
//! y subtotales de lineas de detalle de procesos de venta.
//! El foco está en la facilidad de uso y en aprender Rust, por lo que hay muchas oportunidades de mejora.
//!
//!
use std::{fmt, str::FromStr};

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive, Zero};
use discount::DiscountComputer;
use serde::Serialize;
use tax::{tax_stage, TaxComputer};

pub mod discount;
pub mod tax;

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
///  use baggins::discount::Type;
///  use baggins::DetailCalculator;
///  use crate::baggins::Calculator;
///
///  let mut c = DetailCalculator::new();
///
///  c.add_discount_from_str("22.74", Type::Percentual);
///
///  let r = c.compute_from_str("120.34", "-10", 2);
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
pub enum BagginsError {
    /// Error due to pass a negative unit value to the computer
    NegativeUnitValue(String),

    /// Error due to calculate a negative result
    NegativeResult(String),

    /// Error due to pass a negative quantity
    NegativeQty(String),

    /// Error for not being able to convert a value to [BigDecimal]
    InvalidDecimalValue(String),

    /// Any other unspecified error
    Other(String),
}

impl fmt::Display for BagginsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BagginsError::NegativeUnitValue(ref msg) => write!(
                f,
                "Negative unit value Error: {msg} unit value cannot be negative"
            ),
            BagginsError::NegativeQty(ref value) => {
                write!(f, "Negative quantity value Error: {value}")
            }
            BagginsError::InvalidDecimalValue(msg) => write!(f, " Invalid decimal value {msg}"),
            BagginsError::Other(msg) => write!(f, " Error {msg}"),
            BagginsError::NegativeResult(msg) => {
                write!(f, "Negative result value Error: {msg}")
            }
        }
    }
}

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

#[derive(Debug, Serialize)]
/// will contain the result of the computer of the specified subtotal
pub struct Calculation {
    /// stores the unit value multiplied by the quantity minus the discount
    pub net: BigDecimal,
    /// stores the net plus taxes
    pub brute: BigDecimal,
    /// stores the cumulated tax calculated over net
    pub tax: BigDecimal,
    /// stores the cumulated discount
    pub discount: BigDecimal,
    /// net without discount. Stores the net plus the discounted value
    pub net_wout_disc: BigDecimal,
    /// brute without discount. Stores the brute plus the discounted value
    pub brute_wout_disc: BigDecimal,
    /// tax without discount. Stores the cumulated taxes plus the discounted value
    pub tax_wout_disc: BigDecimal,
    /// stores the unit value
    pub unit_value: BigDecimal,
    /// stores the total discount applied as a percentage
    pub total_discount_percent: BigDecimal,
}

impl Calculation {
    /// Returns a new Calculation with their fields set to zero
    ///
    /// # Example
    ///
    /// ```
    /// use baggins::Calculation;
    ///  let clt = Calculation::new();
    /// ```
    ///  
    pub fn new() -> Self {
        Self {
            net: BigDecimal::zero(),
            brute: BigDecimal::zero(),
            tax: BigDecimal::zero(),
            discount: BigDecimal::zero(),
            net_wout_disc: BigDecimal::zero(),
            brute_wout_disc: BigDecimal::zero(),
            tax_wout_disc: BigDecimal::zero(),
            unit_value: BigDecimal::zero(),
            total_discount_percent: BigDecimal::zero(),
        }
    }

    /// Returns a [CalculationF64] from the original [Calculation]
    /// This may cause precission loss
    ///
    /// # Example
    ///
    /// ```
    /// use baggins::Calculation;
    ///
    /// let clt = Calculation::new();
    /// let clt_f64 = clt.to_f64_calculation();
    /// ```
    ///
    pub fn to_f64_calculation(&self) -> Option<CalculationF64> {
        Some(CalculationF64 {
            net: self.net.to_f64()?,
            brute: self.brute.to_f64()?,
            tax: self.tax.to_f64()?,
            discount: self.discount.to_f64()?,
            net_wout_disc: self.net_wout_disc.to_f64()?,
            brute_wout_disc: self.brute_wout_disc.to_f64()?,
            tax_wout_disc: self.tax_wout_disc.to_f64()?,
            unit_value: self.unit_value.to_f64()?,
            total_discount_percent: self.total_discount_percent.to_f64()?,
        })
    }

    /// Returns a [CalculationString] from the original [Calculation]
    ///
    /// # Example
    ///
    /// ```
    /// use baggins::Calculation;
    ///
    /// let clt = Calculation::new();
    /// let clt_string = clt.to_string_calculation();
    /// ```
    ///
    pub fn to_string_calculation(&self) -> CalculationString {
        CalculationString {
            net: self.net.to_string(),
            brute: self.brute.to_string(),
            tax: self.tax.to_string(),
            discount: self.discount.to_string(),
            net_wout_disc: self.net_wout_disc.to_string(),
            brute_wout_disc: self.brute_wout_disc.to_string(),
            tax_wout_disc: self.tax_wout_disc.to_string(),
            unit_value: self.unit_value.to_string(),
            total_discount_percent: self.total_discount_percent.to_string(),
        }
    }

    /// rounds the value of the Calculation fields to the specified scale
    ///
    /// # Example
    ///
    /// ```
    /// use baggins::discount::Type;
    /// use baggins::DetailCalculator;
    /// use crate::baggins::Calculator;
    ///
    /// let mut c = DetailCalculator::new();
    ///
    /// let result = c.compute_from_str("10.1", "2", 32); // returns a calculation with a max of 32 decimals
    ///
    /// match result {
    ///     Ok(rounded_32_calculation) => {
    ///         let rounded_2_calculation = rounded_32_calculation.round(2); // returns a calculation rounded to 2 decimals
    ///     },
    ///     Err(err) => { panic!("{}", err)},
    /// }
    ///
    /// ```
    pub fn round(&self, scale: i8) -> Self {
        let scale = i64::from(scale);
        Self {
            net: self.net.round(scale),
            tax: self.tax.round(scale),
            discount: self.discount.round(scale),
            brute: self.brute.round(scale),
            net_wout_disc: self.net_wout_disc.round(scale),
            brute_wout_disc: self.brute_wout_disc.round(scale),
            tax_wout_disc: self.tax_wout_disc.round(scale),
            unit_value: self.unit_value.round(scale),
            total_discount_percent: self.total_discount_percent.round(scale),
        }
    }
}

impl Default for Calculation {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Calculation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "net {}, brute {}, tax {}, discount {}, net_wout_disc {}, brute_wout_disc {}, tax_wout_disc {}, unit_value {}, total_discount_percent {} )",
            self.net,
            self.brute,
            self.tax, self.discount, self.net_wout_disc, self.brute_wout_disc, self.tax_wout_disc, self.unit_value, self.total_discount_percent
        )
    }
}

#[derive(Debug, Serialize)]
// stores the values as f64. Some precision may be loss
pub struct CalculationF64 {
    pub net: f64,
    pub brute: f64,
    pub tax: f64,
    pub discount: f64,
    pub net_wout_disc: f64,
    pub brute_wout_disc: f64,
    pub tax_wout_disc: f64,
    pub unit_value: f64,
    pub total_discount_percent: f64,
}

#[derive(Debug, Serialize)]
// stores the field values as string
pub struct CalculationString {
    pub net: String,
    pub brute: String,
    pub tax: String,
    pub discount: String,
    pub net_wout_disc: String,
    pub brute_wout_disc: String,
    pub tax_wout_disc: String,
    pub unit_value: String,
    pub total_discount_percent: String,
}

impl fmt::Display for CalculationF64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "net {}, brute {}, tax {}, discount {}, net_wout_disc {}, brute_wout_disc {}, tax_wout_disc {}, unit_value {}, total_discount_percent {} )",
            self.net,
            self.brute,
            self.tax, self.discount, self.net_wout_disc, self.brute_wout_disc, self.tax_wout_disc, self.unit_value, self.total_discount_percent
        )
    }
}

// A thing able to calculate sales subtotals
pub trait Calculator {
    /// adds a discount to [Calculator].
    ///
    /// # Params
    ///
    /// discount [BigDecimal] the value to discount
    /// discount_type [discount::Type] the type of the discount.
    fn add_discount(
        &mut self,
        discount: BigDecimal,
        discount_type: discount::Type,
    ) -> Option<discount::DiscountError>;

    /// adds a discount to [Calculator] from a [f64] value so expect some precision loss
    /// 
    /// # Params
    ///
    /// discount [f64] the value to discount
    /// discount_type [discount::Type] the type of the discount.
    fn add_discount_from_f64(
        &mut self,
        discount: f64,
        discount_type: discount::Type,
    ) -> Option<discount::DiscountError>;


    /// adds a discount to [`Calculator`] from a [`Into<String>`] value 
    /// 
    /// # Params
    ///
    /// discount [`Into<String>`] the value to discount
    /// discount_type [`discount::Type`] the type of the discount.
    fn add_discount_from_str<S: Into<String>>(
        &mut self,
        discount: S,
        discount_type: discount::Type,
    ) -> Option<discount::DiscountError>;

    /// adds a tax to [Calculator] from a [f64] value so expect some precision loss
    /// 
    /// # Params
    ///
    /// tax [f64] the value to tax
    /// stage [tax::tax_stage::Stage]  the stage where to add the tax
    /// tax_type [tax::Type] the type of the tax.
    fn add_tax_from_f64(
        &mut self,
        tax: f64,
        stage: tax_stage::Stage,
        tax_type: tax::Type,
    ) -> Option<tax::TaxError>;

    /// adds a tax to [Calculator] from a [BigDecimal]
    /// 
    /// # Params
    ///
    /// tax [BigDecimal] the value to tax
    /// stage [tax::tax_stage::Stage]  the stage where to add the tax
    /// tax_type [tax::Type] the type of the tax.
    fn add_tax(
        &mut self,
        tax: BigDecimal,
        stage: tax_stage::Stage,
        tax_type: tax::Type,
    ) -> Option<tax::TaxError>;


    /// adds a tax to [Calculator] from a [String]
    /// 
    /// # Params
    ///
    /// tax [String] the value to tax
    /// stage [tax::tax_stage::Stage]  the stage where to add the tax
    /// tax_type [tax::Type] the type of the tax.
    fn add_tax_from_str<S: Into<String>>(
        &mut self,
        tax: S,
        stage: tax_stage::Stage,
        tax_type: tax::Type,
    ) -> Option<tax::TaxError>;

    /// calculates and produces a [Calculation] from a [BigDecimal] brute value
    /// and a quantity of the same type
    /// 
    /// # Params
    /// 
    /// brute [BigDecimal]  the total from which compute the rest of the values
    /// qty [BigDecimal]  quantity of items sold
    /// scale [i8] decimal scale to round values
    fn compute_from_brute(
        &self,
        brute: BigDecimal,
        qty: BigDecimal,
        scale: i8,
    ) -> Result<Calculation, BagginsError>;

    /// calculates and produces a [Calculation] from a [f64] brute value
    /// and a quantity of the same type
    /// 
    /// # Params
    /// 
    /// brute [f64]  the total from which compute the rest of the values
    /// qty [f64]  quantity of items sold
    /// scale [i8] decimal scale to round values
    fn compute_from_brute_f64(
        &self,
        brute: f64,
        qty: f64,
        scale: i8,
    ) -> Result<Calculation, BagginsError>;

    /// calculates and produces a [Calculation] from a [String] brute value
    /// and a quantity of the same type
    /// 
    /// # Params
    /// 
    /// brute [String]  the total from which compute the rest of the values
    /// qty [String]  quantity of items sold
    /// scale [i8] decimal scale to round values
    fn compute_from_brute_str<S: Into<String>>(
        &self,
        brute: S,
        qty: S,
        scale: i8,
    ) -> Result<Calculation, BagginsError>;

    /// calculates and produces a [Calculation] from a [String] unit value
    /// and a quantity of the same type
    /// 
    /// # Params
    /// 
    /// unit value [String]  the unit value from which compute the rest of the values
    /// qty [String]  quantity of items sold
    /// scale [i8] decimal scale to round values
    fn compute_from_str<S: Into<String>>(
        &self,
        unit_value: S,
        qty: S,
        scale: i8,
    ) -> Result<Calculation, BagginsError>;

    /// calculates and produces a [Calculation] from a [f64] unit value
    /// and a quantity of the same type
    /// 
    /// # Params
    /// 
    /// unit value [f64]  the unit value from which compute the rest of the values
    /// qty [f64]  quantity of items sold
    /// scale [i8] decimal scale to round values
    fn compute_from_f64(
        &self,
        unit_value: f64,
        qty: f64,
        scale: i8,
    ) -> Result<Calculation, BagginsError>;

    /// calculates and produces a [Calculation] from a [BigDecimal] unit value
    /// and a quantity of the same type
    /// 
    /// # Params
    /// 
    /// unit value [BigDecimal]  the unit value from which compute the rest of the values
    /// qty [BigDecimal]  quantity of items sold
    /// scale [i8] decimal scale to round values
    fn compute(
        &self,
        unit_value: BigDecimal,
        qty: BigDecimal,
        scale: i8,
    ) -> Result<Calculation, BagginsError>;
}

/// Able to calculate detail lines.
/// 
/// Delegates the heavy lifting to [tax::ComputedTax] and [discount::ComputedDiscount]
///
/// # Example
///
/// ```
/// use std::str::FromStr;
///
/// use bigdecimal::BigDecimal;
/// use baggins::{discount, tax, Calculator};
/// use baggins::tax::tax_stage::Stage::OverTaxable;
///
/// let mut c = baggins::DetailCalculator::new();
///
/// let err = c.add_discount_from_str("10.0", discount::Type::Percentual);
///
/// assert!(err.is_none(), "{}", format!("{}", err.unwrap()));
///
/// let err = c.add_tax_from_str("16.0", OverTaxable, tax::Type::Percentual);
///
/// assert!(err.is_none(), "{}", format!("{}", err.unwrap()));
///
/// let r = c.compute(
///     BigDecimal::from_str("100.0").unwrap(),
///     BigDecimal::from_str("2.0").unwrap(),
///     32,
/// );
///
/// match r {
///     Ok(calc) => {
///         println!("calc: {}", calc);
///     }
///     Err(e) => {
///         panic!("{e}")
///     }
/// }
/// ```
///
pub struct DetailCalculator {
    tax_handler: tax::ComputedTax,
    discount_handler: discount::ComputedDiscount,
}

impl DetailCalculator {
    /// Creates a new [`DetailCalculator`].
    pub fn new() -> Self {
        Self {
            tax_handler: tax::ComputedTax::new(),
            discount_handler: discount::ComputedDiscount::new(),
        }
    }
}

impl Calculator for DetailCalculator {
    fn add_discount(
        &mut self,
        discount: BigDecimal,
        discount_type: discount::Type,
    ) -> Option<discount::DiscountError> {
        self.discount_handler.add_discount(discount, discount_type)
    }

    fn add_discount_from_f64(
        &mut self,
        discount: f64,
        discount_type: discount::Type,
    ) -> Option<discount::DiscountError> {
        self.discount_handler
            .add_discount_from_f64(discount, discount_type)
    }

    fn add_discount_from_str<S: Into<String>>(
        &mut self,
        discount: S,
        discount_type: discount::Type,
    ) -> Option<discount::DiscountError> {
        self.discount_handler
            .add_discount_from_str(discount, discount_type)
    }

    fn add_tax_from_f64(
        &mut self,
        tax: f64,
        stage: tax_stage::Stage,
        tax_type: tax::Type,
    ) -> Option<tax::TaxError> {
        self.tax_handler.add_tax_from_f64(tax, stage, tax_type)
    }

    fn add_tax(
        &mut self,
        tax: BigDecimal,
        stage: tax_stage::Stage,
        tax_type: tax::Type,
    ) -> Option<tax::TaxError> {
        self.tax_handler.add_tax(tax, stage, tax_type)
    }

    fn add_tax_from_str<S: Into<String>>(
        &mut self,
        tax: S,
        stage: tax_stage::Stage,
        tax_type: tax::Type,
    ) -> Option<tax::TaxError> {
        self.tax_handler.add_tax_from_str(tax, stage, tax_type)
    }

    fn compute_from_brute(
        &self,
        brute: BigDecimal,
        qty: BigDecimal,
        scale: i8,
    ) -> Result<Calculation, BagginsError> {
        let scale = if scale < 0 { 32 } else { scale };

        let r_opt = self.tax_handler.un_tax(brute.clone(), qty.clone());

        match r_opt {
            Ok(net) => {
                let opt_uv = self.discount_handler.un_discount(net, qty.clone());

                match opt_uv {
                    Ok(uv) => self.compute(uv, qty, scale),
                    Err(e) => Err(BagginsError::Other(e.to_string())),
                }
            }
            Err(e) => Err(BagginsError::Other(e.to_string())),
        }
    }

    fn compute_from_brute_f64(
        &self,
        brute: f64,
        qty: f64,
        scale: i8,
    ) -> Result<Calculation, BagginsError> {
        let opt_bt = BigDecimal::from_f64(brute);

        match opt_bt {
            Some(bt) => {
                let opt_q = BigDecimal::from_f64(qty);

                match opt_q {
                    Some(qty) => self.compute_from_brute(bt, qty, scale),
                    None => Err(BagginsError::InvalidDecimalValue(format!("{qty}"))),
                }
            }
            None => Err(BagginsError::InvalidDecimalValue(format!("{brute}"))),
        }
    }

    fn compute_from_brute_str<S: Into<String>>(
        &self,
        brute: S,
        qty: S,
        scale: i8,
    ) -> Result<Calculation, BagginsError> {
        let opt_bt = BigDecimal::from_str(&brute.into());

        match opt_bt {
            Ok(bt) => {
                let opt_q = BigDecimal::from_str(&qty.into());

                match opt_q {
                    Ok(qty) => self.compute_from_brute(bt, qty, scale),
                    Err(e) => Err(BagginsError::InvalidDecimalValue(format!("{e}"))),
                }
            }
            Err(e) => Err(BagginsError::InvalidDecimalValue(format!("{e}"))),
        }
    }

    fn compute_from_str<S: Into<String>>(
        &self,
        unit_value: S,
        qty: S,
        scale: i8,
    ) -> Result<Calculation, BagginsError> {
        let opt_uv = BigDecimal::from_str(unit_value.into().clone().as_str());

        match opt_uv {
            Ok(uv) => {
                let opt_q = BigDecimal::from_str(qty.into().clone().as_str());

                match opt_q {
                    Ok(q) => self.compute(uv.clone(), q.clone(), scale),
                    Err(e) => Err(BagginsError::InvalidDecimalValue(format!("qty {e}"))),
                }
            }
            Err(e) => Err(BagginsError::InvalidDecimalValue(format!(
                "{e} unit value"
            ))),
        }
    }

    fn compute_from_f64(
        &self,
        unit_value: f64,
        qty: f64,
        scale: i8,
    ) -> Result<Calculation, BagginsError> {
        let opt_uv = BigDecimal::from_f64(unit_value);

        match opt_uv {
            Some(uv) => {
                let opt_q = BigDecimal::from_f64(qty);

                match opt_q {
                    Some(qty) => self.compute(uv, qty, scale),
                    None => Err(BagginsError::InvalidDecimalValue(format!("{qty}"))),
                }
            }
            None => Err(BagginsError::InvalidDecimalValue(format!("{unit_value}"))),
        }
    }

    fn compute(
        &self,
        unit_value: BigDecimal,
        qty: BigDecimal,
        scale: i8,
    ) -> Result<Calculation, BagginsError> {
        let scale = if scale < 0 { 32 } else { scale };

        let r = self
            .discount_handler
            .compute(unit_value.clone(), qty.clone());

        match r {
            Ok(d) => {
                let net = &unit_value * &qty - &d;
                let discounted_uv = &net / &qty;

                let r = self.tax_handler.compute(discounted_uv.clone(), qty.clone());

                match r {
                    Ok(tx) => {

                        let r = self.tax_handler.compute(unit_value.clone(), qty.clone());

                        match r {
                            Ok(tx_wout) => {
                                let net_wout = &unit_value * &qty;
                                let calc = Calculation{
                                    brute: &net + &tx,
                                    total_discount_percent: hundred() * &d / &net_wout,
                                    net,
                                    brute_wout_disc: &net_wout + &tx_wout,
                                    net_wout_disc: net_wout,
                                    tax_wout_disc: tx_wout,
                                    discount: d,
                                    tax: tx,
                                    unit_value,
                                };

                                Ok(calc.round(scale))
                            },

                            Err(e) => {
                                Err(BagginsError::Other(format!(
                                    "error calculating tax without discount {e} from unit_value {}, qty {}  scale {scale}",
                                    discounted_uv, qty
                                )))
                            },
                        }
                    },
                    Err(e) => {
                        Err(BagginsError::Other(format!(
                            "error calculating tax {e} from discounted_unit_value {}, qty {}  scale {scale}",
                            discounted_uv, qty
                        )))
                    },
                }
            }

            Err(e) => Err(BagginsError::Other(format!(
                "error calculating discount {e} from unit_value {}, qty {}  scale {scale}",
                unit_value, qty
            ))),
        }
    }
}

impl Default for DetailCalculator {
    fn default() -> Self {
        Self::new()
    }
}
