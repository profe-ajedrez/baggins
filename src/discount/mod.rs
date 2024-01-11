//! discount
//!
//! `discount` module provides ways to calculate discounts.
//!
use std::{fmt, str::FromStr};

use bigdecimal::{BigDecimal, FromPrimitive};

use crate::hundred;

// Different types of discounts are represented here we use the mode identificator to identify them
#[derive(PartialEq)]
pub enum Mode {
    /// It's a discount applied as a percentage over a value as when someone says *a discount of 10%*
    Percentual,

    /// It's a discount applied as an amount over the entirety of the line without consider quantity, as when someone says *a discount of $10 over the total $100*
    AmountLine,

    /// It's a discount applied as an amount over the value of the unit. considers quantity, as when someone says *a discount of $1 by each of the ten oranges*
    AmountUnit,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mode::Percentual => write!(f, "Percentual"),
            Mode::AmountLine => write!(f, "AmountLine"),
            Mode::AmountUnit => write!(f, "AmountUnit"),
        }
    }
}

impl Mode {
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
/// Possible errors of the discount processing
pub enum DiscountError<S: Into<String>> {
    /// a negative value is not allowed, How do you discount 10% of -10?, or the -10% of 10
    NegativeValue(S),

    /// discounts beyond a maximum value is not allowed
    OverMaxDiscount(S),

    /// we work with [BigDecimal] values. Values which cannot be converted will trigger this error
    InvalidDecimal(S),

    /// a discount should be among the allowed modes
    InvalidDiscountMode(S),

    /// something was wrong
    Other(S),
}

impl<S: Into<String> + Clone> fmt::Display for DiscountError<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiscountError::NegativeValue(info) => write!(
                f,
                "Negative value Error. <discountable>, <discount> and <quantity> cannot be negative. {}",
                info.clone().into(),
            ),
            DiscountError::OverMaxDiscount(info) => write!(f, "Over max discount error. {}", info.clone().into()),
            DiscountError::InvalidDecimal(info) => write!(f, "Invalid decimal value error {}", info.clone().into()),
            DiscountError::InvalidDiscountMode(info) => write!(f, "Invalid discount Stage value. {}", info.clone().into()),
            DiscountError::Other(info) => write!(f, "Unknown error! {}", info.clone().into()),
        }
    }
}

/// Represents a thing able to calculates discounts
pub trait Discounter {
    /// adds a f64 value as a discount of the specified mode. Using f64 values may cause some precission loss
    /// because some decimal values only can be represented as an aproximation as floats.
    /// Can return [DiscountError::OverMaxDiscount] [DiscountError::NegativeValue] wrapped in [Option]
    fn add_discount_from_f64(
        &mut self,
        discount: f64,
        discount_mode: Mode,
    ) -> Option<DiscountError<String>>;

    /// adds a string value as a discount of the specified mode. Using string values may cause some speed loss
    /// because they have to be converted.
    /// If the value cannot be converted a [DiscountError::InvalidDecimal] will be returned wrapped in [Option].
    /// Can also return [DiscountError::OverMaxDiscount] [DiscountError::NegativeValue] wrapped in [Option]
    fn add_discount_from_str<S: Into<String>>(
        &mut self,
        discount: S,
        discount_mode: Mode,
    ) -> Option<DiscountError<String>>;

    /// adds a [BigDecimal] value as a discount of the specified mode.
    fn add_discount(
        &mut self,
        discount: BigDecimal,
        discount_mode: Mode,
    ) -> Option<DiscountError<String>>;

    /// Computes the value of the registered discounts applied a [f64] discountable value and a [f64] quantity.
    /// When successful returns a tuple containing the cummulated value of the discount, and the cummulated percentual
    /// discount.
    /// Using f64 values may cause some precission loss because some decimal values only can be represented as an aproximation as floats
    /// Can return [DiscountError::NegativeValue] [DiscountError::OverMaxDiscount]
    fn compute_from_f64(
        &self,
        unit_value: f64,
        qty: f64,
        max_discount_allowed: Option<f64>,
    ) -> Result<(BigDecimal, BigDecimal), DiscountError<String>>;

    /// computes the value of the registered discounts applied a [BigDecimal] discountable value and a [Bigdecimal] quantity.
    /// validating the value of the discount is not over max_discount_allowed if any
    /// When successful returns a tuple containing the cummulated value of the discount, and the cummulated percentual
    /// discount.
    /// Can return [DiscountError::NegativeValue] [DiscountError::OverMaxDiscount]
    fn compute(
        &self,
        unit_value: BigDecimal,
        qty: BigDecimal,
        max_discount_allowed: Option<BigDecimal>,
    ) -> Result<(BigDecimal, BigDecimal), DiscountError<String>>;

    /// computes the value of the registered discounts applied a [Into<String>] discountable value and a [Into<String>] quantity.
    /// When successful returns a tuple containing the cummulated value of the discount, and the cummulated percentual
    /// discount.
    /// Can return [DiscountError::NegativeValue] [DiscountError::OverMaxDiscount]
    fn compute_from_str<S: Into<String>>(
        &self,
        unit_value: S,
        qty: S,
        max_discount_allowed: Option<S>,
    ) -> Result<(BigDecimal, BigDecimal), DiscountError<String>>;

    /// Removes the registered discounts over the discounted value received.
    /// When successful returns a tuple of [BigDecimal] with the undiscounted value, the removed discount value,
    /// and the percentual discount removed.
    /// Can return [DiscountError::NegativeValue]
    fn un_discount(
        &self,
        discounted: BigDecimal,
        qty: BigDecimal,
    ) -> Result<(BigDecimal, BigDecimal, BigDecimal), DiscountError<String>>;

    /// Removes the registered discounts over the discounted [f64] value received.
    /// When using f64 some precission loss can be expected.
    /// When successful returns a tuple of [BigDecimal] with the undiscounted value, the removed discount value,
    /// and the percentual discount removed.
    /// Can return [DiscountError::NegativeValue]
    fn un_discount_from_f64(
        &self,
        discounted: f64,
        qty: f64,
    ) -> Result<(BigDecimal, BigDecimal, BigDecimal), DiscountError<String>>;

    /// Removes the registered discounts over the discounted [Into<String>] value received.
    /// When successful returns a tuple of [BigDecimal] with the undiscounted value, the removed discount value,
    /// and the percentual discount removed.
    /// Can return [DiscountError::NegativeValue]
    fn un_discount_from_str<S: Into<String>>(
        &self,
        discounted: S,
        qty: S,
    ) -> Result<(BigDecimal, BigDecimal, BigDecimal), DiscountError<String>>;

    /// returns the percentual value of an applied discount over a discounted value
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
/// use baggins::discount::{Discounter, DiscountComputer, Mode};
///
/// fn main() {
///     let mut d = DiscountComputer::new();
///
///     let err = d.add_discount(BigDecimal::from_str("10.2").unwrap(), Mode::Percentual);
///     match err {
///         Some(e) => {
///             panic!("{e}")
///         },
///         None => {},
///     }
///
///     let err = d.add_discount_from_str("10.56", Mode::AmountUnit);
///     match err {
///         Some(e) => {
///             panic!("{e}")
///         },
///         None => {},
///     }
///
///     let err = d.add_discount(BigDecimal::from_str("1.5").unwrap(), Mode::AmountLine);
///     match err {
///         Some(e) => {
///             panic!("{e}")
///         },
///         None => {},
///     }
///
///     let res = d.compute_from_f64(100.0, 1.0, Some(100.0f64));
///     
///     match res {
///         Ok(disc) => {
///             let expected = BigDecimal::from_str("22.26").unwrap();
///             
///             if disc.0 != expected {
///                 panic!("expected {:?}. Got {:?}", expected, disc);
///             }
///         },
///         Err(e) => {
///             panic!("{e}");
///         },
///     }
/// }
///```
///
pub struct DiscountComputer {
    percentual: BigDecimal,
    amount_line: BigDecimal,
    amount_unit: BigDecimal,
}

impl DiscountComputer {
    pub fn new() -> Self {
        Self {
            percentual: crate::zero(),
            amount_line: crate::zero(),
            amount_unit: crate::zero(),
        }
    }
}

impl Default for DiscountComputer {
    fn default() -> Self {
        Self::new()
    }
}

impl Discounter for DiscountComputer {
    fn add_discount_from_f64(
        &mut self,
        discount: f64,
        discount_mode: Mode,
    ) -> Option<DiscountError<String>> {
        if discount < 0.0f64 {
            return Some(DiscountError::NegativeValue(format!(
                "negative discount {discount}"
            )));
        }

        if discount > 100.0f64 && discount_mode == Mode::Percentual {
            return Some(DiscountError::OverMaxDiscount(format!(
                "percentual discount over 100%. {discount}"
            )));
        }

        match discount_mode {
            Mode::Percentual => {
                self.percentual =
                    &self.percentual + BigDecimal::from_f64(discount).unwrap_or(crate::zero())
            }
            Mode::AmountLine => {
                self.amount_line =
                    &self.amount_line + BigDecimal::from_f64(discount).unwrap_or(crate::zero())
            }
            Mode::AmountUnit => {
                self.amount_unit =
                    &self.amount_unit + BigDecimal::from_f64(discount).unwrap_or(crate::zero())
            }
        }

        None
    }

    fn add_discount_from_str<S: Into<String>>(
        &mut self,
        discount: S,
        discount_mode: Mode,
    ) -> Option<DiscountError<String>> {
        let d = discount.into();
        match BigDecimal::from_str(&d) {
            Ok(discount) => self.add_discount(discount.clone(), discount_mode),
            Err(err) => Some(DiscountError::InvalidDecimal(format!(
                "discount {}  err {}",
                d, err
            ))),
        }
    }

    fn add_discount(
        &mut self,
        discount: BigDecimal,
        discount_mode: Mode,
    ) -> Option<DiscountError<String>> {
        if discount < crate::zero() {
            return Some(DiscountError::NegativeValue(format!(
                "negative discount {}",
                discount
            )));
        }

        if discount > crate::hundred() && discount_mode == Mode::Percentual {
            return Some(DiscountError::OverMaxDiscount(format!(
                "percentual discount over 100%. {}",
                discount
            )));
        }

        match discount_mode {
            Mode::Percentual => self.percentual = &self.percentual + discount,
            Mode::AmountLine => self.amount_line = &self.amount_line + discount,
            Mode::AmountUnit => self.amount_unit = &self.amount_unit + discount,
        }

        None
    }

    fn compute_from_f64(
        &self,
        unit_value: f64,
        qty: f64,
        max_discount_allowed: Option<f64>,
    ) -> Result<(BigDecimal, BigDecimal), DiscountError<String>> {
        let unit_value = BigDecimal::from_f64(unit_value).unwrap_or(crate::inverse());
        let qty = BigDecimal::from_f64(qty).unwrap_or(crate::inverse());

        let max_discount_allowed = BigDecimal::from_f64(max_discount_allowed.unwrap_or(100.0f64))
            .unwrap_or(crate::inverse());

        self.compute(unit_value, qty, Some(max_discount_allowed))
    }

    fn compute(
        &self,
        unit_value: BigDecimal,
        qty: BigDecimal,
        max_discount_allowed: Option<BigDecimal>,
    ) -> Result<(BigDecimal, BigDecimal), DiscountError<String>> {
        let max_discount_allowed = max_discount_allowed.unwrap_or(crate::hundred());

        if max_discount_allowed < crate::zero() {
            return Err(DiscountError::NegativeValue(format!(
                "negative <max_discount_allowed> {}",
                max_discount_allowed
            )));
        }

        if unit_value < crate::zero() {
            return Err(DiscountError::NegativeValue(format!(
                "negative <unit_value> {}",
                unit_value
            )));
        }

        if qty < crate::zero() {
            return Err(DiscountError::NegativeValue(format!(
                "negative <qty> {}",
                qty
            )));
        }

        let discount_value = &unit_value * &qty * &self.percentual / crate::hundred()
            + &self.amount_unit * &qty
            + &self.amount_line;

        if discount_value > max_discount_allowed {
            return Err(DiscountError::OverMaxDiscount(format!(
                " discount_value {}   max_discount_allowed {}",
                discount_value, max_discount_allowed
            )));
        }

        let percentual_discount = (&unit_value * &qty - &discount_value) / crate::hundred();

        if percentual_discount > crate::hundred() {
            return Err(DiscountError::OverMaxDiscount(format!(
                "percentual_discount {}",
                percentual_discount
            )));
        }

        if percentual_discount < crate::zero() {
            return Err(DiscountError::NegativeValue(format!(
                "percentual_discount {}",
                percentual_discount
            )));
        }

        Ok((discount_value, percentual_discount))
    }

    fn compute_from_str<S: Into<String>>(
        &self,
        unit_value: S,
        qty: S,
        max_discount_allowed: Option<S>,
    ) -> Result<(BigDecimal, BigDecimal), DiscountError<String>> {
        match BigDecimal::from_str(&unit_value.into()) {
            Ok(unit_value) => match BigDecimal::from_str(&qty.into()) {
                Ok(qty) => match max_discount_allowed {
                    Some(max_discount_allowed) => {
                        match BigDecimal::from_str(&max_discount_allowed.into()) {
                            Ok(max_discount_allowed) => {
                                self.compute(unit_value, qty, Some(max_discount_allowed))
                            }
                            Err(err) => Err(DiscountError::InvalidDecimal(format!("{}", err))),
                        }
                    }
                    None => self.compute(unit_value, qty, None),
                },
                Err(err) => Err(DiscountError::InvalidDecimal(format!("{}", err))),
            },
            Err(err) => Err(DiscountError::InvalidDecimal(format!("{}", err))),
        }
    }

    fn un_discount(
        &self,
        discounted: BigDecimal,
        qty: BigDecimal,
    ) -> Result<(BigDecimal, BigDecimal, BigDecimal), DiscountError<String>> {
        if discounted < crate::zero() {
            return Err(DiscountError::NegativeValue(format!(
                "negative <discounted> {}",
                discounted
            )));
        }

        if qty < crate::zero() {
            return Err(DiscountError::NegativeValue(format!(
                "negative <qty> {}",
                qty
            )));
        }

        let percentual = if self.percentual > crate::zero() {
            self.percentual.clone()
        } else {
            crate::one()
        };

        let discountable = (&discounted + &self.amount_line) / &percentual * crate::hundred()
            + &qty * &self.amount_unit;
        let percentual_discount = (&discountable - &discounted) * crate::hundred() / &discountable;

        Ok((
            discountable.clone(),
            discountable - &discounted,
            percentual_discount,
        ))
    }

    fn un_discount_from_f64(
        &self,
        discounted: f64,
        qty: f64,
    ) -> Result<(BigDecimal, BigDecimal, BigDecimal), DiscountError<String>> {
        self.un_discount(
            BigDecimal::from_f64(discounted).unwrap_or(crate::inverse()),
            BigDecimal::from_f64(qty).unwrap_or(crate::inverse()),
        )
    }

    fn un_discount_from_str<S: Into<String>>(
        &self,
        discounted: S,
        qty: S,
    ) -> Result<(BigDecimal, BigDecimal, BigDecimal), DiscountError<String>> {
        match BigDecimal::from_str(&discounted.into()) {
            Ok(discounted) => match BigDecimal::from_str(&qty.into()) {
                Ok(qty) => self.un_discount(discounted, qty),
                Err(err) => Err(DiscountError::InvalidDecimal(format!("{}", err))),
            },
            Err(err) => Err(DiscountError::InvalidDecimal(format!("{}", err))),
        }
    }

    fn ratio(&self, discounted: BigDecimal, discount: BigDecimal) -> BigDecimal {
        (&discounted - &discount) * crate::hundred() / &discounted
    }
}
