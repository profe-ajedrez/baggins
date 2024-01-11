//! tax
//!
//! `tax` module provides ways to calculate taxes.
//!
use std::{fmt, str::FromStr};

use bigdecimal::{BigDecimal, FromPrimitive};

#[derive(PartialEq)]
/// The tax type
/// A tax type could be percentual or a fixed amount, and the fixed amount tax
/// could be by each unit or by everything being sold
///
/// # Example
///
/// ```
/// use baggins::tax;
///
/// let tax_type = tax::Mode::Percentual;
/// ```
pub enum Mode {
    /// Percentual represents a tax calculated as percent value over a taxable
    Percentual,

    /// Tax calculated as a fixed amount over a taxable only
    /// one time without consider quantity.
    ///
    /// If you sell apples and there is a tax which is charged regardless of
    /// the number of apples sold, plus a $3 tax that applies to both 1 and
    /// 10 apples sold, then that is a line item tax
    AmountLine,

    /// Tax calculated as a fixed value over the unit value of the product
    /// being sold.
    /// Applies for each unit being sold
    ///
    /// If you sell apples and there is a tax which is chaged for every apple that
    /// is an amount unit tax
    AmountUnit,
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
/// Possible errors of the tax processing
pub enum TaxError<S: Into<String>> {
    /// a negative value is not allowed, How do you tax 10% of -10?, or the -10% of 10
    NegativeValue(S),

    /// discounts beyond a maximum value is not allowed
    OverMaxDiscount(S),

    /// we work with [BigDecimal] values. Values which cannot be converted will trigger this error
    InvalidDecimal(S),

    /// a tax should be among the allowed modes
    InvalidDiscountMode(S),

    DivisionByZero(S),

    /// something was wrong
    Other(S),
}

impl<S: Into<String> + Clone> fmt::Display for TaxError<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaxError::NegativeValue(info) => write!(
                f,
                "Negative value Error. <discountable>, <tax> and <quantity> cannot be negative. {}",
                info.clone().into(),
            ),
            TaxError::OverMaxDiscount(info) => {
                write!(f, "Over max tax error. {}", info.clone().into())
            }
            TaxError::InvalidDecimal(info) => {
                write!(f, "Invalid decimal value error {}", info.clone().into())
            }
            TaxError::InvalidDiscountMode(info) => {
                write!(f, "Invalid tax Stage value. {}", info.clone().into())
            }
            TaxError::DivisionByZero(info) => write!(
                f,
                "division by zero when calculating  {}",
                info.clone().into()
            ),
            TaxError::Other(info) => write!(f, "Unknown error! {}", info.clone().into()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stage {
    /// Taxes that are calculated directly on the value of the products
    OverTaxable,

    /// Taxes that are calculated on the values of the products plus the
    /// over taxable tax calculated for them
    OverTax,

    /// taxes calculated the same as overtaxables, but are not considered
    /// for the calculation of overtaxes.
    OverTaxIgnorable,
}

/// Represents when a tax should be calculated.
/// There are 3 stages in which a tax could be calculated
///
/// 1 directly on the values of the products being sold, we call these taxes
/// over taxables
///
/// 2 on the value obtained from applying overtaxable taxes, we call these
/// overtaxes and they are the typical case of tax on tax
///
/// 3 are calculated the same as overtaxable taxes, but they are not considered
/// for the calculation of overtax taxes, we call these ignorable overtaxes
impl Stage {
    /// returns an [`Option<Stage>`] over an [i8] argument where a value of
    ///
    /// 0 returns an Some(Stage::OverTaxable)
    ///
    /// 1 returns an Some(Stage::OverTax)
    ///
    /// 2 returns an some(Stage::OverTaxIgnorable)
    ///
    /// Other values returns None
    ///
    /// ```
    /// use baggins::tax;
    ///
    /// let stage = tax::Stage::from_i8(0);
    /// ```        
    pub fn from_i8(stage: i8) -> Option<Self> {
        if stage == 0 {
            return Some(Stage::OverTaxable);
        }

        if stage == 1 {
            return Some(Stage::OverTax);
        }

        if stage == 2 {
            return Some(Stage::OverTaxIgnorable);
        }

        None
    }
}

// Represents a thing able to stand as a tax stage and calculate over its recorded taxes
pub trait Stager {
    /// adds a BigDecimal value as a percentual tax to the stage.
    /// Could return [TaxError::NegativeValue] boxed in an [Option]
    fn add_percentual(&mut self, percent: BigDecimal) -> Option<TaxError<String>>;

    /// adds a BigDecimal value as an amount unit tax to the stage
    /// Could return [TaxError::NegativeValue] boxed in an [Option]
    fn add_amount_by_qty(&mut self, amount: BigDecimal) -> Option<TaxError<String>>;

    /// adds a BigDecimal value as an amount line tax to the stage
    /// Could return [TaxError::NegativeValue] boxed in an [Option]
    fn add_amount_by_line(&mut self, amount: BigDecimal) -> Option<TaxError<String>>;

    /// calculates the stage taxes from BigDecimal taxable and quantity
    /// Could return [TaxError::NegativeValue]
    fn tax(&mut self, taxable: BigDecimal, qty: BigDecimal)
        -> Result<BigDecimal, TaxError<String>>;

    /// adds a f64 value as a percentual tax to the stage. This could cause precision loss
    /// Could return [TaxError::NegativeValue] boxed in an [Option]    
    fn add_percentual_from_f64(&mut self, percent: f64) -> Option<TaxError<String>>;

    /// adds a f64 value as an amount unit tax to the stage. This could cause precision loss
    /// Could return [TaxError::InvalidDecimal] [TaxError::NegativeValue] boxed in an [Option]    
    fn add_amount_by_qty_from_f64(&mut self, amount: f64) -> Option<TaxError<String>>;

    /// adds a f64 value as an amount line tax to the stage. This could cause precision loss
    /// Could return [TaxError::NegativeValue] boxed in an [Option]
    fn add_amount_by_line_from_f64(&mut self, amount: f64) -> Option<TaxError<String>>;

    /// calculates the stage taxes from f64 taxable and quantity
    /// Could return [TaxError::NegativeValue]
    fn tax_from_f64(&mut self, taxable: f64, qty: f64) -> Result<BigDecimal, TaxError<String>>;

    /// calculates the stage taxes from [String] taxable and quantity
    /// Could return [TaxError::NegativeValue]
    fn tax_from_str<S: Into<String>>(
        &mut self,
        taxable: S,
        qty: S,
    ) -> Result<BigDecimal, TaxError<String>>;

    /// adds a [String] value as a percentual tax to the stage.
    /// Could return [TaxError::InvalidDecimal] [TaxError::NegativeValue] boxed in an [Option]
    fn add_percentual_from_str<S: Into<String>>(&mut self, percent: S) -> Option<TaxError<String>>;

    /// adds a [String] value as an amount unit tax to the stage.
    /// Could return [TaxError::InvalidDecimal] [TaxError::NegativeValue] boxed in an [Option]
    fn add_amount_by_qty_from_str<S: Into<String>>(
        &mut self,
        amount: S,
    ) -> Option<TaxError<String>>;

    /// adds a [String] value as an amount line tax to the stage.
    /// Could return [TaxError::InvalidDecimal] [TaxError::NegativeValue] boxed in an [Option]
    fn add_amount_by_line_from_str<S: Into<String>>(
        &mut self,
        amount: S,
    ) -> Option<TaxError<String>>;

    /// returns the cumulative percentual value of the percentual taxes of the stage
    /// could return [`BigDecimal::Zero`]
    fn percent(&self) -> BigDecimal;

    /// returns the cumulative value of the amount line taxes of the stage
    /// could return [`BigDecimal::Zero`]
    fn amount_line(&self) -> BigDecimal;

    /// returns the cumulative value of the amount unit taxes of the stage
    /// could return [`BigDecimal::Zero`]
    fn amount_by_qty(&self) -> BigDecimal;
}

#[derive(Clone)]
/// Able to store tax data belonging to a given stage and make calculations with them
///
/// # Example
///
/// ```rust
/// use baggins::tax::{TaxComputer, Taxer, Stage, Mode};
/// use bigdecimal::{BigDecimal, FromPrimitive};
/// use std::str::FromStr;
///
/// let mut taxer = baggins::tax::TaxComputer::new();
///
/// let err = taxer.add_tax_from_f64(18.0, Stage::OverTaxable, Mode::Percentual);
/// assert!(err.is_none(), "error triggered adding first f64 tax");
///
/// let err = taxer.add_tax_from_f64(10.0, Stage::OverTaxable, Mode::Percentual);
/// assert!(err.is_none(), "error triggered adding second f64 tax");
///
/// let err = taxer.add_tax_from_f64(0.5, Stage::OverTaxable, Mode::AmountUnit);
/// assert!(err.is_none(), "error triggered adding third f64 tax");
///
/// let r = taxer.tax_from_f64(24.576855, 4.0);
///
/// match r {
///     Ok(tax) => {
///         let expected =
///             BigDecimal::from_str("29.52607759999999814226612215861678123474121093750000")
///                 .unwrap();
///         assert_eq!(tax, expected);
///         println!("calculated_tax: {}", tax);
///     }
///
///     Err(e) => {
///         panic!("{e}")
///     }
/// }
/// ```
pub struct TaxStage {
    percentuals: BigDecimal,
    amount_line: BigDecimal,
    amount_unit: BigDecimal,
}

impl TaxStage {
    /// returns a TaxStage ready to use
    ///
    /// # Example
    ///
    /// ```
    /// use baggins::tax::TaxStage;
    ///
    /// let tax_st = TaxStage::new();
    /// ```
    pub fn new() -> Self {
        Self {
            percentuals: crate::zero(),
            amount_line: crate::zero(),
            amount_unit: crate::zero(),
        }
    }
}

impl Default for TaxStage {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TaxStage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "percentuals {} amount_line {} amount_unit {}",
            self.percentuals, self.amount_line, self.amount_unit
        )
    }
}

impl Stager for TaxStage {
    fn add_percentual(&mut self, percent: BigDecimal) -> Option<TaxError<String>> {
        if percent < crate::zero() {
            return Some(TaxError::NegativeValue(format!(
                "negative value adding percentual tax {}",
                percent
            )));
        }

        self.percentuals = &self.percentuals + percent;
        None
    }

    fn add_amount_by_qty(&mut self, amount: BigDecimal) -> Option<TaxError<String>> {
        if amount < crate::zero() {
            return Some(TaxError::NegativeValue(format!(
                "negative value adding amount tax {}",
                amount
            )));
        }

        self.amount_unit = &self.amount_unit + amount;
        None
    }

    fn add_amount_by_line(&mut self, amount: BigDecimal) -> Option<TaxError<String>> {
        if amount < crate::zero() {
            return Some(TaxError::NegativeValue(format!(
                "negative value adding amount line tax {}",
                amount
            )));
        }

        self.amount_line = &self.amount_line + amount;
        None
    }

    fn tax(
        &mut self,
        taxable: BigDecimal,
        qty: BigDecimal,
    ) -> Result<BigDecimal, TaxError<String>> {
        if taxable < crate::zero() {
            return Err(TaxError::NegativeValue(format!(
                "negative taxable at calculating registered taxes{}",
                taxable
            )));
        }

        if qty < crate::zero() {
            return Err(TaxError::NegativeValue(format!(
                "negative quantity at calculating registered taxes {}",
                qty
            )));
        }

        if taxable == crate::zero() {
            return Ok(crate::zero());
        }

        // println!("{} {}", taxable, self);

        Ok(
            (&taxable * &self.percentuals / crate::hundred() + &self.amount_unit) * &qty
                + &self.amount_line,
        )
    }

    fn add_percentual_from_f64(&mut self, percent: f64) -> Option<TaxError<String>> {
        self.add_percentual(BigDecimal::from_f64(percent).unwrap_or(crate::inverse()))
    }

    fn add_amount_by_qty_from_f64(&mut self, amount: f64) -> Option<TaxError<String>> {
        self.add_amount_by_qty(BigDecimal::from_f64(amount).unwrap_or(crate::inverse()))
    }

    fn add_amount_by_line_from_f64(&mut self, amount: f64) -> Option<TaxError<String>> {
        self.add_amount_by_line(BigDecimal::from_f64(amount).unwrap_or(crate::inverse()))
    }

    fn tax_from_f64(&mut self, taxable: f64, qty: f64) -> Result<BigDecimal, TaxError<String>> {
        self.tax(
            BigDecimal::from_f64(taxable).unwrap_or(crate::inverse()),
            BigDecimal::from_f64(qty).unwrap_or(crate::inverse()),
        )
    }

    fn percent(&self) -> BigDecimal {
        self.percentuals.clone()
    }

    fn amount_line(&self) -> BigDecimal {
        self.amount_line.clone()
    }

    fn amount_by_qty(&self) -> BigDecimal {
        self.amount_unit.clone()
    }

    fn tax_from_str<S: Into<String>>(
        &mut self,
        taxable: S,
        qty: S,
    ) -> Result<BigDecimal, TaxError<String>> {
        let taxable = taxable.into();
        let qty = qty.into();

        match BigDecimal::from_str(&taxable) {
            Ok(taxable) => match BigDecimal::from_str(&qty) {
                Ok(qty) => self.tax(taxable, qty),
                Err(err) => Err(TaxError::InvalidDecimal(err.to_string())),
            },
            Err(err) => Err(TaxError::InvalidDecimal(err.to_string())),
        }
    }

    fn add_percentual_from_str<S: Into<String>>(&mut self, percent: S) -> Option<TaxError<String>> {
        let percent = percent.into();

        match BigDecimal::from_str(&percent) {
            Ok(percent) => self.add_percentual(percent),
            Err(err) => Some(TaxError::InvalidDecimal(format!(
                "invalid percent value adding percentual from str {}",
                err
            ))),
        }
    }

    fn add_amount_by_qty_from_str<S: Into<String>>(
        &mut self,
        amount: S,
    ) -> Option<TaxError<String>> {
        let amount = amount.into();

        match BigDecimal::from_str(&amount) {
            Ok(amount) => self.add_amount_by_qty(amount),
            Err(err) => Some(TaxError::InvalidDecimal(format!(
                "invalid amount value adding amount by qty from str {}",
                err
            ))),
        }
    }

    fn add_amount_by_line_from_str<S: Into<String>>(
        &mut self,
        amount: S,
    ) -> Option<TaxError<String>> {
        let amount = amount.into();

        match BigDecimal::from_str(&amount) {
            Ok(amount) => self.add_amount_by_line(amount),
            Err(err) => Some(TaxError::InvalidDecimal(format!(
                "invalid amount value adding amount line from str {}",
                err
            ))),
        }
    }
}

/// A handler to the taxes calculation stages is represented here
pub trait Taxer {
    fn over_taxables(&self) -> impl Stager;
    fn over_taxes(&self) -> impl Stager;
    fn over_tax_ignorables(&self) -> impl Stager;

    /// adds a [BigDecimal] value of the specified [Mode] to the specified [Stage]
    /// Could returns [TaxError::NegativeValue] boxed in an [Option]
    fn add_tax(
        &mut self,
        tax: BigDecimal,
        stage: Stage,
        tax_type: Mode,
    ) -> Option<TaxError<String>>;

    /// adds a [f64] value of the specified [Mode] to the specified [Stage]
    /// Using f64 values may cause some precission loss
    /// because some decimal values only can be represented as an aproximation as floats.
    /// Could returns [TaxError::NegativeValue] boxed in an [Option]
    fn add_tax_from_f64(
        &mut self,
        tax: f64,
        stage: Stage,
        tax_type: Mode,
    ) -> Option<TaxError<String>>;

    /// adds a [Into<String>] value of the specified [Mode] to the specified [Stage]    
    /// Could returns [TaxError::NegativeValue] [TaxError::InvalidDecimal] boxed in an [Option]
    fn add_tax_from_str<S: Into<String>>(
        &mut self,
        tax: S,
        stage: Stage,
        tax_type: Mode,
    ) -> Option<TaxError<String>>;

    /// returns the calculated cummulated tax value for the specified [BigDecimal] unit_value.
    /// Could returns [TaxError::NegativeValue]
    fn tax(
        &mut self,
        unit_value: BigDecimal,
        qty: BigDecimal,
    ) -> Result<BigDecimal, TaxError<String>>;

    /// returns the calculated cummulated tax value for the specified [f64] unit_value.
    /// Using f64 values may cause some precission loss
    /// because some decimal values only can be represented as an aproximation as floats.
    /// Could returns [TaxError::NegativeValue]
    fn tax_from_f64(&mut self, unit_value: f64, qty: f64) -> Result<BigDecimal, TaxError<String>>;

    /// returns the calculated cummulated tax value for the specified [Into<String>] unit_value.
    /// Could returns [TaxError::NegativeValue] [TaxError::InvalidDecimal]
    fn tax_from_str<S: Into<String>>(
        &mut self,
        unit_value: S,
        qty: S,
    ) -> Result<BigDecimal, TaxError<String>>;

    /// removes the calculated cummulated tax value for the specified [BigDecimal] taxed.
    /// returning the value over the cummulated taxes were calculated.
    /// Could returns [TaxError::NegativeValue]
    fn un_tax(&self, taxed: BigDecimal, qty: BigDecimal) -> Result<BigDecimal, TaxError<String>>;

    /// removes the calculated cummulated tax value for the specified [f64] taxed.
    /// returning the value over the cummulated taxes were calculated.
    /// Using f64 values may cause some precission loss
    /// because some decimal values only can be represented as an aproximation as floats.
    /// Could returns [TaxError::NegativeValue]
    fn un_tax_from_f64(&self, taxed: f64, qty: f64) -> Result<BigDecimal, TaxError<String>>;

    /// removes the calculated cummulated tax value for the specified [Into<String>] taxed.
    /// returning the value over the cummulated taxes were calculated.
    /// Could returns [TaxError::NegativeValue] [TaxError::InvalidDecimal]
    fn un_tax_from_str<S: Into<String>>(
        &self,
        taxed: S,
        qty: S,
    ) -> Result<BigDecimal, TaxError<String>>;

    /// returns the [BigDecimal] percentual value of the specified tax applied to the specified taxable
    /// Could returns [TaxError::DivisionByZero]
    fn ratio(taxed: BigDecimal, tax: BigDecimal) -> Result<BigDecimal, TaxError<String>> {
        if taxed == crate::zero() && tax == crate::zero() {
            return Err(TaxError::DivisionByZero(
                "taxed and tax values are zero. couldnt divide by zero".to_string(),
            ));
        }

        Ok(crate::hundred() * &tax / (&taxed + &tax))
    }

    /// returns the value of the specified tax applied over the specified taxable and quantity.
    /// Could returns [TaxError::NegativeValue]
    fn line_tax(
        &self,
        taxable: BigDecimal,
        qty: BigDecimal,
        value: BigDecimal,
        mode: Mode,
    ) -> Result<BigDecimal, TaxError<String>> {
        if taxable < crate::zero() {
            return Err(TaxError::NegativeValue("negative taxable".to_string()));
        }

        if qty < crate::zero() {
            return Err(TaxError::NegativeValue("negative quantity".to_string()));
        }

        match mode {
            Mode::Percentual => Ok(&taxable * &qty * &value / crate::hundred()),
            Mode::AmountLine => Ok(&qty * &value),
            Mode::AmountUnit => Ok(value),
        }
    }

    /// returns the value of the specified [Into<String>] tax applied over the specified taxable and quantity.
    /// Could returns [TaxError::NegativeValue] [TaxError::InvalidDecimal]
    fn line_tax_from_str<S: Into<String>>(
        &self,
        taxable: S,
        qty: S,
        value: S,
        mode: Mode,
    ) -> Result<BigDecimal, TaxError<String>> {
        let taxable = taxable.into();
        let qty = qty.into();
        let value = value.into();

        match BigDecimal::from_str(&taxable) {
            Ok(taxable) => match BigDecimal::from_str(&qty) {
                Ok(qty) => match BigDecimal::from_str(&value) {
                    Ok(value) => self.line_tax(taxable, qty, value, mode),
                    Err(err) => Err(TaxError::InvalidDecimal(err.to_string())),
                },
                Err(err) => Err(TaxError::InvalidDecimal(err.to_string())),
            },
            Err(err) => Err(TaxError::InvalidDecimal(err.to_string())),
        }
    }

    /// returns the value of the specified [f64] tax applied over the specified taxable and quantity.
    /// Could returns [TaxError::NegativeValue]
    fn line_tax_from_f64(
        &self,
        taxable: f64,
        qty: f64,
        value: f64,
        mode: Mode,
    ) -> Result<BigDecimal, TaxError<String>> {
        self.line_tax(
            BigDecimal::from_f64(taxable).unwrap_or(crate::inverse()),
            BigDecimal::from_f64(qty).unwrap_or(crate::inverse()),
            BigDecimal::from_f64(value).unwrap_or(crate::inverse()),
            mode,
        )
    }
}

pub struct TaxComputer {
    over_taxable: TaxStage,
    over_tax: TaxStage,
    over_tax_ignorable: TaxStage,
}

impl TaxComputer {
    /// returns a ComputerTax stage handler
    ///
    /// # Examples
    ///
    /// ```
    /// use baggins::tax::TaxComputer;
    /// use baggins::tax::TaxStage;
    ///
    /// let mut tax_calculator = TaxComputer::new();
    /// ```
    ///
    pub fn new() -> Self {
        Self {
            over_taxable: TaxStage::default(),
            over_tax: TaxStage::default(),
            over_tax_ignorable: TaxStage::default(),
        }
    }
}

impl Default for TaxComputer {
    fn default() -> Self {
        Self::new()
    }
}

impl Taxer for TaxComputer {
    fn over_taxables(&self) -> impl Stager {
        self.over_taxable.clone()
    }

    fn over_taxes(&self) -> impl Stager {
        self.over_tax.clone()
    }

    fn over_tax_ignorables(&self) -> impl Stager {
        self.over_tax_ignorable.clone()
    }

    fn add_tax(&mut self, tax: BigDecimal, stage: Stage, mode: Mode) -> Option<TaxError<String>> {
        match stage {
            Stage::OverTaxable => match mode {
                Mode::Percentual => self.over_taxable.add_percentual(tax),
                Mode::AmountLine => self.over_taxable.add_amount_by_line(tax),
                Mode::AmountUnit => self.over_taxable.add_amount_by_qty(tax),
            },
            Stage::OverTax => match mode {
                Mode::Percentual => self.over_tax.add_percentual(tax),
                Mode::AmountLine => self.over_tax.add_amount_by_line(tax),
                Mode::AmountUnit => self.over_tax.add_amount_by_qty(tax),
            },
            Stage::OverTaxIgnorable => match mode {
                Mode::Percentual => self.over_tax_ignorable.add_percentual(tax),
                Mode::AmountLine => self.over_tax_ignorable.add_amount_by_line(tax),
                Mode::AmountUnit => self.over_tax_ignorable.add_amount_by_qty(tax),
            },
        }
    }

    fn add_tax_from_f64(
        &mut self,
        tax: f64,
        stage: Stage,
        tax_type: Mode,
    ) -> Option<TaxError<String>> {
        self.add_tax(
            BigDecimal::from_f64(tax).unwrap_or(crate::inverse()),
            stage,
            tax_type,
        )
    }

    fn add_tax_from_str<S: Into<String>>(
        &mut self,
        tax: S,
        stage: Stage,
        tax_type: Mode,
    ) -> Option<TaxError<String>> {
        let tax = tax.into();

        match BigDecimal::from_str(&tax) {
            Ok(tax) => self.add_tax(tax, stage, tax_type),
            Err(err) => Some(TaxError::InvalidDecimal(err.to_string())),
        }
    }

    fn tax(
        &mut self,
        unit_value: BigDecimal,
        qty: BigDecimal,
    ) -> Result<BigDecimal, TaxError<String>> {
        // if unit_value < crate::zero() {
        //     return Err(TaxError::NegativeValue(format!("unit_value {}", unit_value)))
        // }

        // if qty < crate::zero() {
        //     return Err(TaxError::NegativeValue(format!("quantity {}", qty)))
        // }

        // let net = &unit_value * &qty;
        match self.over_taxable.tax(unit_value.clone(), qty.clone()) {
            Ok(tax_over_taxable) => match self
                .over_tax
                .tax(&tax_over_taxable + &unit_value, qty.clone())
            {
                Ok(over_tax) => {
                    match self.over_tax_ignorable.tax(unit_value.clone(), qty.clone()) {
                        Ok(over_tax_ignorable) => {
                            Ok(&tax_over_taxable + &over_tax + &over_tax_ignorable)
                        }
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    fn tax_from_f64(&mut self, unit_value: f64, qty: f64) -> Result<BigDecimal, TaxError<String>> {
        self.tax(
            BigDecimal::from_f64(unit_value).unwrap_or(crate::inverse()),
            BigDecimal::from_f64(qty).unwrap_or(crate::inverse()),
        )
    }

    fn tax_from_str<S: Into<String>>(
        &mut self,
        unit_value: S,
        qty: S,
    ) -> Result<BigDecimal, TaxError<String>> {
        let unit_value = unit_value.into();
        let qty = qty.into();

        match BigDecimal::from_str(&unit_value) {
            Ok(unit_value) => match BigDecimal::from_str(&qty) {
                Ok(qty) => self.tax(unit_value, qty),
                Err(err) => Err(TaxError::InvalidDecimal(format!(
                    " qty {} err {}",
                    qty, err
                ))),
            },
            Err(err) => Err(TaxError::InvalidDecimal(format!(
                " unit_value {}  err {}",
                unit_value, err
            ))),
        }
    }

    /// removes the calculated cummulated tax value for the specified [BigDecimal] taxed.
    /// returning the [BigDecimal] value over the cummulated taxes were calculated.
    /// Could returns [TaxError::NegativeValue]
    ///
    /// This implementation uses the next equation to un tax the taxed value
    ///
    /// taxed – b * d + b + e + h - c * (d + 1) - f - i  /  a * d + a + g + d + 1
    ///
    /// Where
    ///
    /// a = over_taxable.percentuals / 100
    ///
    /// b = over_taxable.amount_by_qty() * qty
    ///
    /// c = over_taxable.amount_line
    ///
    ///
    /// d = over_tax.percentuals / 100
    ///
    /// e = over_tax.amount_by_qty() * qty
    ///
    /// f = over_tax.amount_line
    ///
    ///
    /// g = over_tax_ignorable.percentuals / 100
    ///
    /// h = over_tax_ignorable.amount_by_qty() * qty
    ///
    /// i = over_tax_ignorable.amount_line
    ///
    ///
    fn un_tax(&self, taxed: BigDecimal, qty: BigDecimal) -> Result<BigDecimal, TaxError<String>> {
        if qty < crate::zero() {
            return Err(TaxError::NegativeValue(format!("qty {}", qty)));
        }

        let a = &self.over_taxable.percentuals / crate::hundred();
        let b = &self.over_taxable.amount_by_qty() * &qty;
        let c = &self.over_taxable.amount_line;
        let d = &self.over_tax.percentuals / crate::hundred();
        let e = &self.over_tax.amount_by_qty() * &qty;
        let f = &self.over_tax.amount_line;
        let g = &self.over_tax_ignorable.percentuals / crate::hundred();
        let h = &self.over_tax_ignorable.amount_by_qty() * &qty;
        let i = &self.over_tax_ignorable.amount_line;

        let numerator = &taxed - &b * &d + b + e + h - c * (&d + crate::one()) - f - i;
        let denominator = &a + &d + &a + &g + &d + crate::one();

        Ok(numerator / denominator)
    }

    /// removes the calculated cummulated tax value for the specified [f64] taxed.
    /// returning the [BigDecimal] value over the cummulated taxes were calculated.
    /// Using f64 may cause some precission loss
    /// Could returns [TaxError::NegativeValue]
    ///
    /// This implementation uses the next equation to un tax the taxed value
    ///
    /// taxed – b * d + b + e + h - c * (d + 1) - f - i  /  a * d + a + g + d + 1
    ///
    /// Where
    ///
    /// a = over_taxable.percentuals / 100
    ///
    /// b = over_taxable.amount_by_qty() * qty
    ///
    /// c = over_taxable.amount_line
    ///
    ///
    /// d = over_tax.percentuals / 100
    ///
    /// e = over_tax.amount_by_qty() * qty
    ///
    /// f = over_tax.amount_line
    ///
    ///
    /// g = over_tax_ignorable.percentuals / 100
    ///
    /// h = over_tax_ignorable.amount_by_qty() * qty
    ///
    /// i = over_tax_ignorable.amount_line
    ///
    ///
    fn un_tax_from_f64(&self, taxed: f64, qty: f64) -> Result<BigDecimal, TaxError<String>> {
        self.un_tax(
            BigDecimal::from_f64(taxed).unwrap_or(crate::inverse()),
            BigDecimal::from_f64(qty).unwrap_or(crate::inverse()),
        )
    }

    /// removes the calculated cummulated tax value for the specified [Into<String>] taxed.
    /// returning the [BigDecimal] value over the cummulated taxes were calculated.
    /// Could returns [TaxError::NegativeValue] [TaxError::InvalidDecimal]
    ///
    /// This implementation uses the next equation to un tax the taxed value
    ///
    /// taxed – b * d + b + e + h - c * (d + 1) - f - i  /  a * d + a + g + d + 1
    ///
    /// Where
    ///
    /// a = over_taxable.percentuals / 100
    ///
    /// b = over_taxable.amount_by_qty() * qty
    ///
    /// c = over_taxable.amount_line
    ///
    ///
    /// d = over_tax.percentuals / 100
    ///
    /// e = over_tax.amount_by_qty() * qty
    ///
    /// f = over_tax.amount_line
    ///
    ///
    /// g = over_tax_ignorable.percentuals / 100
    ///
    /// h = over_tax_ignorable.amount_by_qty() * qty
    ///
    /// i = over_tax_ignorable.amount_line
    ///
    ///
    fn un_tax_from_str<S: Into<String>>(
        &self,
        taxed: S,
        qty: S,
    ) -> Result<BigDecimal, TaxError<String>> {
        let taxed = taxed.into();
        let qty = qty.into();

        match BigDecimal::from_str(&taxed) {
            Ok(taxed) => match BigDecimal::from_str(&qty) {
                Ok(qty) => self.un_tax(taxed, qty),
                Err(err) => Err(TaxError::InvalidDecimal(format!(
                    "invalid taxed {} err  {}",
                    taxed, err
                ))),
            },
            Err(err) => Err(TaxError::InvalidDecimal(format!(
                "invalid taxed {} err  {}",
                taxed, err
            ))),
        }
    }
}
