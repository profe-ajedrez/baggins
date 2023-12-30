use std::{fmt, str::FromStr};

use bigdecimal::{BigDecimal, FromPrimitive, Zero};

use crate::hundred;

// Different types of discounts are represented here
pub enum Type {
    /// It's a discount applied as a tasa over a value as when someone says *a discount of 10%*
    Percentual,

    /// It's a discount applied as an amount over the entirety of the line without consider quantity, as when someone says *a discount of $10 over the total $100*
    AmountLine,

    /// It's a discount applied as an amount over the value of the unit. considers quantity, as when someone says *a discount of $1 by each of the ten oranges*
    AmountUnit,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Percentual => write!(f, "Percentual"),
            Type::AmountLine => write!(f, "AmountLine"),
            Type::AmountUnit => write!(f, "AmountUnit"),
        }
    }
}

impl Type {
    pub fn from_i8(r#type: i8) -> Option<Self> {
        if r#type == 0 {
            return Some(Self::Percentual);
        }

        if r#type == 1 {
            return Some(Self::AmountLine);
        }

        if r#type == 2 {
            return Some(Self::AmountUnit);
        }

        None
    }
}

#[derive(Debug)]
pub enum DiscountError {
    NegativeDiscountable(String),
    NegativeDiscount(String),
    NegativePercent(f64),
    NegativeAmountByUnit(f64),
    NegativeAmountByLine(f64),
    NegativeQuantity(String),
    OverMaxDiscount(String),
    InvalidDecimal(String),
    InvalidDiscountStage,

    Other,
}

// Allow the use of "{}" format specifier
impl fmt::Display for DiscountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DiscountError::NegativeDiscountable(ref discountable) => write!(
                f,
                "Negative discountable Error: {discountable} discountable cannot be negative"
            ),
            DiscountError::NegativeDiscount(ref discount) => write!(
                f,
                "Negative discount Error: {discount} discountable cannot be negative"
            ),
            DiscountError::NegativePercent(ref value) => {
                write!(f, "Negative percentual discount value Error: {value}")
            }
            DiscountError::NegativeAmountByUnit(ref value) => {
                write!(f, "Negative amount by unit discount value Error: {value}")
            }
            DiscountError::NegativeAmountByLine(ref value) => {
                write!(f, "Negative amount by line discount value Error: {value}")
            }
            DiscountError::NegativeQuantity(ref msg) => write!(f, "{msg}"),
            DiscountError::OverMaxDiscount(ref msg) => write!(f, "Over max discount {msg}"),
            DiscountError::InvalidDecimal(ref value) => write!(f, "Invalid decimal value {value}"),
            DiscountError::InvalidDiscountStage => write!(f, "Invalid discount Stage value"),
            DiscountError::Other => write!(f, "Unknown error!"),
        }
    }
}

/// Represents a thing able to calculates discounts
pub trait DiscountComputer {
    fn add_discount_from_f64(&mut self, discount: f64, discount_type: Type) -> Option<DiscountError>;
    fn add_discount_from_str<S: Into<String>>(
        &mut self,
        discount: S,
        discount_type: Type,
    ) -> Option<DiscountError>;
    fn add_discount(&mut self, discount: BigDecimal, discount_type: Type) -> Option<DiscountError>;
    fn compute_from_f64(&self, unit_value: f64, qty: f64) -> Result<BigDecimal, DiscountError>;
    fn compute(&self, unit_value: BigDecimal, qty: BigDecimal)
        -> Result<BigDecimal, DiscountError>;
    fn compute_from_str<S: Into<String>>(
        &self,
        unit_value: S,
        qty: S,
    ) -> Result<BigDecimal, DiscountError>;
    fn un_discount(
        &self,
        discounted: BigDecimal,
        qty: BigDecimal,
    ) -> Result<BigDecimal, DiscountError>;
    fn un_discount_from_f64(&self, discounted: f64, qty: f64) -> Result<BigDecimal, DiscountError>;
    fn un_discount_from_str<S: Into<String>>(
        &self,
        discounted: S,
        qty: S,
    ) -> Result<BigDecimal, DiscountError>;

    fn ratio(&self, discounted: BigDecimal, discount: BigDecimal) -> BigDecimal {
        hundred() * &discount / (&discounted + &discount)
    }
}

/// calculates discounts
///
/// # Example
///
/// ```
/// use std::str::FromStr;
/// use bigdecimal::BigDecimal;
/// use calculus::discount::{ComputedDiscount, DiscountComputer, Type};
///
/// fn main() {
///     let mut d = ComputedDiscount::new();
///
///     let err = d.add_discount(BigDecimal::from_str("10.2").unwrap(), Type::Percentual);
///     match err {
///         Some(e) => {
///             panic!("{e}")
///         },
///         None => {},
///     }
///
///     let err = d.add_str_discount("10.56", Type::AmountUnit);
///     match err {
///         Some(e) => {
///             panic!("{e}")
///         },
///         None => {},
///     }
///
///     let err = d.add_discount(BigDecimal::from_str("1.5").unwrap(), Type::AmountLine);
///     match err {
///         Some(e) => {
///             panic!("{e}")
///         },
///         None => {},
///     }
///
///     let res = d.compute_from_f64(100.0, 1.0);
///     
///     match res {
///         Ok(disc) => {
///             let expected = BigDecimal::from_str("22.26").unwrap();
///             
///             if disc != expected {
///                 panic!("expected {}. Got {}", expected, disc);
///             }
///         },
///         Err(e) => {
///             panic!("{e}");
///         },
///     }
/// }
///```
///
pub struct ComputedDiscount {
    percentual: BigDecimal,
    amount_line: BigDecimal,
    amount_unit: BigDecimal,
    max_discount: BigDecimal,
}

impl ComputedDiscount {
    pub fn new() -> Self {
        Self {
            percentual: BigDecimal::zero(),
            amount_line: BigDecimal::zero(),
            amount_unit: BigDecimal::zero(),
            max_discount: BigDecimal::zero(),
        }
    }
}

impl Default for ComputedDiscount {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscountComputer for ComputedDiscount {
    fn add_discount_from_str<S: Into<String>>(
        &mut self,
        discount: S,
        discount_type: Type,
    ) -> Option<DiscountError> {
        let opt_d = BigDecimal::from_str(discount.into().as_str());

        match opt_d {
            Ok(d) => self.add_discount(d, discount_type),
            Err(e) => Some(DiscountError::InvalidDecimal(format!("{e}"))),
        }
    }

    fn add_discount_from_f64(&mut self, discount: f64, discount_type: Type) -> Option<DiscountError> {
        let opt_d = BigDecimal::from_f64(discount);

        match opt_d {
            Some(d) => {
                if discount < 0.0 {
                    return Some(DiscountError::NegativeDiscount(format!(
                        "negative discount {discount} of type {}",
                        discount_type
                    )));
                }

                match discount_type {
                    Type::Percentual => {
                        self.percentual = &self.percentual + d;
                    }

                    Type::AmountUnit => {
                        self.amount_unit = &self.amount_unit + d;
                    }

                    Type::AmountLine => {
                        self.amount_line = &self.amount_line + d;
                    }
                }

                None
            }

            None => Some(DiscountError::InvalidDecimal(format!(
                "invalid decimal value for make a discount {discount}"
            ))),
        }
    }

    fn add_discount(&mut self, discount: BigDecimal, discount_type: Type) -> Option<DiscountError> {
        if discount < BigDecimal::zero() {
            return Some(DiscountError::NegativeDiscount(format!(
                "negative discount {discount} of type {}",
                discount_type
            )));
        }

        match discount_type {
            Type::Percentual => {
                self.percentual = &self.percentual + discount;
            }
            Type::AmountLine => {
                self.amount_line = &self.amount_line + discount;
            }
            Type::AmountUnit => {
                self.amount_unit = &self.amount_unit + discount;
            }
        }

        None
    }

    fn compute_from_f64(&self, unit_value: f64, qty: f64) -> Result<BigDecimal, DiscountError> {
        if unit_value < 0.0 {
            return Err(DiscountError::NegativeDiscountable(format!(
                "negative discountable unit_value {}",
                unit_value
            )));
        }

        if qty < 0.0 {
            return Err(DiscountError::NegativeQuantity(format!(
                "negative quantity: {qty}"
            )));
        }

        let uv = BigDecimal::from_f64(unit_value).unwrap();
        let q = BigDecimal::from_f64(qty).unwrap();

        let mut mx_disc = self.max_discount.clone();

        if mx_disc <= BigDecimal::zero() {
            mx_disc = hundred().clone()
        }

        let max_discount_value = &uv * &mx_disc / hundred();
        let discounted =
            (&uv * &self.percentual / hundred() + &self.amount_unit) * &q + &self.amount_line;

        if discounted < BigDecimal::zero() {
            return Err(DiscountError::NegativeDiscount(format!(
                " calculated discount was negative {}",
                discounted
            )));
        }

        if discounted > max_discount_value {
            return Err(DiscountError::OverMaxDiscount(format!(
                "[maxdisocuntpercent:{}] [maxdiscount:{}] [calculated discount:{}]",
                mx_disc, max_discount_value, discounted
            )));
        }

        Ok(discounted)
    }

    fn compute(
        &self,
        unit_value: BigDecimal,
        qty: BigDecimal,
    ) -> Result<BigDecimal, DiscountError> {
        if unit_value < BigDecimal::zero() {
            return Err(DiscountError::NegativeDiscountable(format!(
                "negative discountable unit_value {}",
                unit_value
            )));
        }

        if qty < BigDecimal::zero() {
            return Err(DiscountError::NegativeQuantity(format!(
                "negative quantity: {}",
                qty
            )));
        }

        let mut mx_disc = self.max_discount.clone();

        if mx_disc <= BigDecimal::zero() {
            mx_disc = hundred().clone()
        }

        let max_discount_value = &unit_value * &mx_disc / hundred();
        let discounted = (&unit_value * &self.percentual / hundred() + &self.amount_unit) * &qty
            + &self.amount_line;

        if discounted > max_discount_value {
            return Err(DiscountError::OverMaxDiscount(format!(
                "[maxdisocuntpercent:{}] [maxdiscount:{}] [calculated discount:{}]",
                &self.max_discount, max_discount_value, discounted
            )));
        }

        Ok(discounted)
    }

    fn compute_from_str<S: Into<String>>(
        &self,
        unit_value: S,
        qty: S,
    ) -> Result<BigDecimal, DiscountError> {
        let opt_uv = BigDecimal::from_str(unit_value.into().as_str());

        match opt_uv {
            Ok(uv) => {
                let opt_qty = BigDecimal::from_str(qty.into().as_str());

                match opt_qty {
                    Ok(q) => self.compute(uv, q),
                    Err(e) => Err(DiscountError::InvalidDecimal(format!(
                        "invalid qty in compute_from_str {e}"
                    ))),
                }
            }
            Err(e) => Err(DiscountError::InvalidDecimal(format!(
                " invalid unit value in compute_from_str: {e}"
            ))),
        }
    }

    fn un_discount(
        &self,
        discounted: BigDecimal,
        qty: BigDecimal,
    ) -> Result<BigDecimal, DiscountError> {
        let wout_line = discounted + &self.amount_line;

        if wout_line < BigDecimal::zero() {
            return Err(DiscountError::NegativeDiscountable(format!(
                "negative discountable founded when un_discounting amount_line discount  {}",
                wout_line
            )));
        }

        let wout_unit = wout_line + (&self.amount_unit * &qty);

        if wout_unit < BigDecimal::zero() {
            return Err(DiscountError::NegativeDiscountable(format!(
                "negative discountable founded when un_discounting amount_unit discount {}",
                wout_unit
            )));
        }

        let original_discountable = wout_unit / (hundred() - &self.percentual) * hundred() / &qty;

        Ok(original_discountable)
    }

    fn un_discount_from_f64(&self, discounted: f64, qty: f64) -> Result<BigDecimal, DiscountError> {
        if discounted < 0.0 {
            return Err(DiscountError::NegativeDiscount(format!(
                "inputed discounted value id negative [{discounted}]"
            )));
        }

        if qty < 0.0 {
            return Err(DiscountError::NegativeQuantity(format!(
                "negative quantity: {qty}"
            )));
        }

        let disc = BigDecimal::from_f64(discounted).unwrap();
        let q = BigDecimal::from_f64(qty).unwrap();

        let wout_line = disc - &self.amount_line;

        if wout_line < BigDecimal::zero() {
            return Err(DiscountError::NegativeDiscountable(format!(
                "negative discountable founded when un_discounting amount_line discount  {}",
                wout_line
            )));
        }

        let wout_unit = wout_line - (&self.amount_unit * &q);

        if wout_unit < BigDecimal::zero() {
            return Err(DiscountError::NegativeDiscountable(format!(
                "negative discountable founded when un_discounting amount_unit discount {}",
                wout_unit
            )));
        }

        let original_discountable = wout_unit / (hundred() - &self.percentual) * hundred() / &q;

        Ok(original_discountable)
    }

    fn un_discount_from_str<S: Into<String>>(
        &self,
        discounted: S,
        qty: S,
    ) -> Result<BigDecimal, DiscountError> {
        let opt_uv = BigDecimal::from_str(discounted.into().as_str());

        match opt_uv {
            Ok(uv) => {
                let opt_qty = BigDecimal::from_str(qty.into().as_str());

                match opt_qty {
                    Ok(q) => self.compute(uv, q),
                    Err(e) => Err(DiscountError::InvalidDecimal(format!(
                        "invalid qty in un_discount_from_str {e}"
                    ))),
                }
            }
            Err(e) => Err(DiscountError::InvalidDecimal(format!(
                " invalid unit value in un_discount_from_str: {e}"
            ))),
        }
    }
}
