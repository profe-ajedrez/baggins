use bigdecimal::{BigDecimal, ToPrimitive};
use std::{fmt, str::FromStr};

/// The tax type
/// A tax type could be percentual or a fixed amount, and the fixed amount tax
/// could be by each unit or by everything being sold
///
/// # Example
///
/// ```
/// use calculus::tax;
///
/// let tax_type = tax::Type::Percentual;
/// ```
pub enum Type {
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
/// Represents possible errors in the computing of taxes
pub enum TaxError {
    NegativeTaxable(f64),
    NegativeTax(String),
    NegativeQty(f64),
    NegativePercent(f64),
    NegativeAmountByUnit(f64),
    NegativeAmountByLine(f64),
    InvalidDecimal(String),
    InvalidTaxStage,
    Other,
}

impl fmt::Display for TaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TaxError::NegativeTaxable(ref taxable) => write!(
                f,
                "Negative taxable Error: {taxable} taxable cannot be negative"
            ),
            TaxError::NegativeTax(ref tax) => {
                write!(f, "Negative tax Error: {tax} taxable cannot be negative")
            }
            TaxError::NegativeQty(ref value) => write!(f, "Negative quantity value Error: {value}"),
            TaxError::NegativePercent(ref value) => {
                write!(f, "Negative percentual tax value Error: {value}")
            }
            TaxError::NegativeAmountByUnit(ref value) => {
                write!(f, "Negative amount by unit tax value Error: {value}")
            }
            TaxError::NegativeAmountByLine(ref value) => {
                write!(f, "Negative amount by line tax value Error: {value}")
            }
            TaxError::InvalidDecimal(ref value) => write!(f, "Invalid decimal value {value}"),
            TaxError::InvalidTaxStage => write!(f, "Invalid Tax Stage value"),
            TaxError::Other => write!(f, "Unknown error!"),
        }
    }
}

/// Represents the time at which a tax should be calculated.
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
pub mod tax_stage {
    use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive, Zero};

    use crate::{hundred, one};

    use super::TaxError;

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

    impl Stage {
        /// returns an Option<Stage> over an i8 argument where a value of
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
        /// use calculus::tax;
        ///
        /// let stage = tax::tax_stage::Stage::from_i8(0);
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

    pub trait Stager {
        /// adds a BigDecimal value as a percentual tax to the stage
        fn add_percentual(&mut self, percent: BigDecimal) -> Option<TaxError>;

        /// adds a BigDecimal value as an amount unit tax to the stage
        fn add_amount_by_qty(&mut self, amount: BigDecimal) -> Option<TaxError>;

        /// adds a BigDecimal value as an amount line tax to the stage
        fn add_amount_by_line(&mut self, amount: BigDecimal) -> Option<TaxError>;

        /// calculates the stage taxes from BigDecimal taxable and quantity
        fn tax(&self, taxable: BigDecimal, qty: BigDecimal) -> Result<BigDecimal, TaxError>;

        /// adds a f64 value as a percentual tax to the stage. This could cause precision loss
        fn add_percentual_from_f64(&mut self, percent: f64) -> Option<TaxError>;

        /// adds a f64 value as an amount unit tax to the stage. This could cause precision loss
        fn add_amount_by_qty_from_f64(&mut self, amount: f64) -> Option<TaxError>;

        /// adds a f64 value as an amount line tax to the stage. This could cause precision loss
        fn add_amount_by_line_from_f64(&mut self, amount: f64) -> Option<TaxError>;

        /// calculates the stage taxes from f64 taxable and quantity
        fn tax_from_f64(&self, taxable: f64, qty: f64) -> Result<BigDecimal, TaxError>;

        /// returns the cumulative percentual value of the percentual taxes of the stage
        fn percent(&self) -> BigDecimal;

        /// returns the cumulative value of the amount line taxes of the stage
        fn amount_line(&self) -> BigDecimal;

        /// returns the cumulative value of the amount unit taxes of the stage
        fn amount_by_qty(&self) -> BigDecimal;

        /// removes stage taxes from the taxed BigDecimal value
        fn un_tax(&self, tax: BigDecimal, qty: BigDecimal) -> Result<BigDecimal, TaxError>;

        /// removes stage taxes from the taxed f64 value
        fn un_tax_from_f64(&self, tax: f64, qty: f64) -> Result<BigDecimal, TaxError>;
    }

    /// Able to store tax data belonging to a given stage and make calculations
    /// on it
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
        /// use calculus::tax;
        ///
        /// let tax_st = tax::tax_stage::TaxStage::new();
        /// ```
        pub fn new() -> Self {
            Self {
                percentuals: BigDecimal::zero(),
                amount_line: BigDecimal::zero(),
                amount_unit: BigDecimal::zero(),
            }
        }
    }

    impl Default for TaxStage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Stager for TaxStage {
        fn add_percentual_from_f64(&mut self, percent: f64) -> Option<TaxError> {
            let opt_dec = BigDecimal::from_f64(percent);

            match opt_dec {
                Some(p) => {
                    self.percentuals = &self.percentuals + p;
                    None
                }

                None => Some(TaxError::InvalidDecimal(format!("{percent}"))),
            }
        }

        fn add_amount_by_qty_from_f64(&mut self, amount: f64) -> Option<TaxError> {
            let opt_dec = BigDecimal::from_f64(amount);

            match opt_dec {
                Some(a) => {
                    self.amount_unit = &self.amount_unit + a;
                    None
                }

                None => Some(TaxError::InvalidDecimal(format!("{amount}"))),
            }
        }

        fn add_amount_by_line_from_f64(&mut self, amount: f64) -> Option<TaxError> {
            let opt_dec = BigDecimal::from_f64(amount);

            match opt_dec {
                Some(a) => {
                    self.amount_line = &self.amount_line + a;
                    None
                }

                None => Some(TaxError::InvalidDecimal(format!("{amount}"))),
            }
        }

        fn tax_from_f64(&self, taxable: f64, qty: f64) -> Result<BigDecimal, TaxError> {
            if taxable < 0.0 {
                return Err(TaxError::NegativeTaxable(taxable));
            }

            if qty < 0.0 {
                return Err(TaxError::NegativeQty(qty));
            }

            let txble = BigDecimal::from_f64(taxable).unwrap();
            let qt = BigDecimal::from_f64(qty).unwrap();

            Ok(
                (&self.percentuals / hundred() * txble + &self.amount_unit) * qt
                    + &self.amount_line,
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

        fn add_percentual(&mut self, percent: BigDecimal) -> Option<TaxError> {
            let opt_p = percent.to_f64();

            match opt_p {
                Some(p) => {
                    if percent < BigDecimal::zero() {
                        return Some(TaxError::NegativePercent(p));
                    }

                    self.percentuals = &self.percentuals + percent;
                    None
                }

                None => Some(TaxError::InvalidDecimal(format!(
                    "invalid decimal value for percentual tax: {}",
                    percent
                ))),
            }
        }

        fn add_amount_by_qty(&mut self, amount: BigDecimal) -> Option<TaxError> {
            let opt_a = amount.to_f64();

            match opt_a {
                Some(a) => {
                    if amount < BigDecimal::zero() {
                        return Some(TaxError::NegativePercent(a));
                    }

                    self.amount_unit = &self.amount_unit + amount;
                    None
                }

                None => Some(TaxError::InvalidDecimal(format!(
                    "invalid decimal value for amount by qty tax: {}",
                    amount
                ))),
            }
        }

        fn add_amount_by_line(&mut self, amount: BigDecimal) -> Option<TaxError> {
            let opt_a = amount.to_f64();

            match opt_a {
                Some(a) => {
                    if amount < BigDecimal::zero() {
                        return Some(TaxError::NegativePercent(a));
                    }

                    self.amount_line = &self.amount_line + amount;
                    None
                }

                None => Some(TaxError::InvalidDecimal(format!(
                    "invalid decimal value for amount by qty tax: {}",
                    amount
                ))),
            }
        }

        fn tax(&self, taxable: BigDecimal, qty: BigDecimal) -> Result<BigDecimal, TaxError> {
            let opt_tx = taxable.to_f64();

            match opt_tx {
                Some(txbl) => {
                    let opt_qty = qty.to_f64();

                    match opt_qty {
                        Some(q) => {
                            if txbl < 0.0 {
                                return Err(TaxError::NegativeTaxable(txbl));
                            }

                            if q < 0.0 {
                                return Err(TaxError::NegativeQty(q));
                            }

                            Ok(
                                (&self.percentuals / hundred() * taxable + &self.amount_unit) * qty
                                    + &self.amount_line,
                            )
                        }
                        None => Err(TaxError::InvalidDecimal(format!(
                            "invalid decimal value for quantity {}",
                            qty
                        ))),
                    }
                }
                None => Err(TaxError::InvalidDecimal(format!(
                    "invalid decimal value for taxable {}",
                    taxable
                ))),
            }
        }

        fn un_tax(&self, tax: BigDecimal, qty: BigDecimal) -> Result<BigDecimal, TaxError> {
            let opt_tax = tax.to_f64();

            match opt_tax {
                Some(tx) => {
                    let opt_qty = qty.to_f64();

                    match opt_qty {
                        Some(q) => {
                            if q < 0.0 {
                                return Err(TaxError::NegativeQty(q));
                            }

                            if tx < 0.0 {
                                return Err(TaxError::NegativeTax(format!("{}", tx)));
                            }

                            let wout_amount_line = tax - self.amount_line();
                            let wout_amount_unit = wout_amount_line - self.amount_by_qty() * qty;
                            let percent = one() + &self.percentuals / hundred();
                            let untaxed = wout_amount_unit / percent;

                            if untaxed < BigDecimal::zero() {
                                return Err(TaxError::NegativeTax(format!(
                                    "untexed was calculated as a negative value {}",
                                    untaxed
                                )));
                            }

                            Ok(untaxed)
                        }

                        None => Err(TaxError::InvalidDecimal(format!(
                            "invalid decimal value for quantity {}",
                            qty
                        ))),
                    }
                }

                None => Err(TaxError::InvalidDecimal(format!(
                    "invalid decimal value for tax {}",
                    tax
                ))),
            }
        }

        fn un_tax_from_f64(&self, tax: f64, qty: f64) -> Result<BigDecimal, TaxError> {
            let opt_tx = BigDecimal::from_f64(tax);

            match opt_tx {
                Some(tx) => {
                    let opt_qty = BigDecimal::from_f64(qty);

                    match opt_qty {
                        Some(q) => {
                            if tax < 0.0 {
                                return Err(TaxError::NegativeTax(format!(
                                    "untexed was calculated as a negative value {tax}"
                                )));
                            }

                            if qty < 0.0 {
                                return Err(TaxError::NegativeQty(qty));
                            }

                            let wout_amount_line = tx - self.amount_line();
                            let wout_amount_unit = wout_amount_line - self.amount_by_qty() * q;
                            let percent = one() + &self.percentuals / hundred();
                            let untaxed = wout_amount_unit / percent;

                            if untaxed < BigDecimal::zero() {
                                return Err(TaxError::NegativeTax(format!(
                                    "untexed was calculated as a negative value {}",
                                    untaxed
                                )));
                            }

                            Ok(untaxed)
                        }
                        None => Err(TaxError::InvalidDecimal(format!(
                            "invalid decimal value for quantity {}",
                            qty
                        ))),
                    }
                }
                None => Err(TaxError::InvalidDecimal(format!(
                    "invalid decimal value for taxable {}",
                    tax
                ))),
            }
        }
    }
}

use crate::{hundred, tax::tax_stage::Stager};

/// A handler to the taxes calculation stage is represented here
pub trait TaxComputer {
    fn add_tax_from_f64(
        &mut self,
        tax: f64,
        stage: tax_stage::Stage,
        tax_type: Type,
    ) -> Option<TaxError>;
    fn add_tax(
        &mut self,
        tax: BigDecimal,
        stage: tax_stage::Stage,
        tax_type: Type,
    ) -> Option<TaxError>;
    fn add_tax_from_str<S: Into<String>>(
        &mut self,
        tax: S,
        stage: tax_stage::Stage,
        tax_type: Type,
    ) -> Option<TaxError>;

    fn compute_from_f64(&self, unit_value: f64, qty: f64) -> Result<BigDecimal, TaxError>;

    fn compute_from_str<S: Into<String>>(
        &self,
        unit_value: S,
        qty: S,
    ) -> Result<BigDecimal, TaxError>;

    fn compute(&self, unit_value: BigDecimal, qty: BigDecimal) -> Result<BigDecimal, TaxError>;

    fn un_tax(&self, taxed: BigDecimal, qty: BigDecimal) -> Result<BigDecimal, TaxError>;

    fn un_tax_from_f64(&self, taxed: f64, qty: f64) -> Result<BigDecimal, TaxError>;

    fn un_tax_from_str<S: Into<String>>(&self, taxed: S, qty: S) -> Result<BigDecimal, TaxError>;

    fn ratio(&self, taxed: BigDecimal, tax: BigDecimal) -> BigDecimal {
        hundred() * &tax / (&taxed + &tax)
    }
}

/// Is a handler to the taxes calculation stages. 
///
/// # Example
///
/// ```
/// use std::str::FromStr;  
/// use bigdecimal::BigDecimal;  
/// use calculus::tax::TaxComputer;  
/// use calculus::tax::Type;  
/// use calculus::tax::tax_stage::Stage;  
///
///     let mut tax_calculator = calculus::tax::ComputedTax::new();
///
///     let err = tax_calculator.add_tax_from_f64(18.0, Stage::OverTaxable, Type::Percentual);
///     assert!(err.is_some(), "error triggered adding first f64 tax {:?}", err);
///
///     let err = tax_calculator.add_tax_from_f64(10.0, Stage::OverTaxable, Type::Percentual);
///     assert!(err.is_some(), "error triggered adding second f64 tax");
///
///     let err = tax_calculator.add_tax_from_f64(0.5, Stage::OverTaxable, Type::AmountUnit);
///     assert!(err.is_some(), "error triggered adding third f64 tax");
///
///     let r = tax_calculator.compute_from_f64(24.576855, 4.0);
///
///     match r {
///         Ok(tax) => {
///             let expected = BigDecimal::from_str("29.52607759999999814226612215861678123474121093750000").unwrap();
///             assert_eq!(tax, expected);
///             println!("calculated_tax: {}", tax);
///         }
///
///         Err(e) => {
///             panic!("{e}")
///         }
///     }
/// ```
///
pub struct ComputedTax {
    over_taxable: tax_stage::TaxStage,
    over_tax: tax_stage::TaxStage,
    over_tax_ignorable: tax_stage::TaxStage,
}

impl ComputedTax {
    /// returns a ComputerTax stage handler
    ///
    /// # Examples
    ///
    /// ```
    /// use calculus::tax::TaxComputer;
    /// let mut tax_calculator = calculus::tax::ComputedTax::new();
    /// ```
    ///
    pub fn new() -> Self {
        Self {
            over_taxable: tax_stage::TaxStage::new(),
            over_tax: tax_stage::TaxStage::new(),
            over_tax_ignorable: tax_stage::TaxStage::new(),
        }
    }
}

impl Default for ComputedTax {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxComputer for ComputedTax {
    fn add_tax_from_f64(
        &mut self,
        tax: f64,
        stage: tax_stage::Stage,
        tax_type: Type,
    ) -> Option<TaxError> {
        match stage {
            tax_stage::Stage::OverTaxable => match tax_type {
                Type::Percentual => {
                    self.over_taxable.add_percentual_from_f64(tax);
                }

                Type::AmountLine => {
                    self.over_taxable.add_amount_by_line_from_f64(tax);
                }

                Type::AmountUnit => {
                    self.over_taxable.add_amount_by_qty_from_f64(tax);
                }
            },

            tax_stage::Stage::OverTax => match tax_type {
                Type::Percentual => {
                    self.over_tax.add_percentual_from_f64(tax);
                }

                Type::AmountLine => {
                    self.over_tax.add_amount_by_line_from_f64(tax);
                }

                Type::AmountUnit => {
                    self.over_tax.add_amount_by_qty_from_f64(tax);
                }
            },

            tax_stage::Stage::OverTaxIgnorable => match tax_type {
                Type::Percentual => {
                    self.over_tax_ignorable.add_percentual_from_f64(tax);
                }

                Type::AmountLine => {
                    self.over_tax_ignorable.add_amount_by_line_from_f64(tax);
                }

                Type::AmountUnit => {
                    self.over_tax_ignorable.add_amount_by_qty_from_f64(tax);
                }
            },
        }

        Some(TaxError::InvalidTaxStage)
    }

    fn add_tax_from_str<S: Into<String>>(
        &mut self,
        tax: S,
        stage: tax_stage::Stage,
        tax_type: Type,
    ) -> Option<TaxError> {
        let opt_tx = BigDecimal::from_str(tax.into().as_str());

        match opt_tx {
            Ok(tx) => self.add_tax(tx, stage, tax_type),
            Err(e) => Some(TaxError::InvalidDecimal(format!("{e}"))),
        }
    }

    fn add_tax(
        &mut self,
        tax: BigDecimal,
        stage: tax_stage::Stage,
        tax_type: Type,
    ) -> Option<TaxError> {
        match stage {
            tax_stage::Stage::OverTaxable => match tax_type {
                Type::Percentual => self.over_taxable.add_percentual(tax),

                Type::AmountLine => self.over_taxable.add_amount_by_line(tax),

                Type::AmountUnit => self.over_taxable.add_amount_by_qty(tax),
            },

            tax_stage::Stage::OverTax => match tax_type {
                Type::Percentual => self.over_tax.add_percentual(tax),

                Type::AmountLine => self.over_tax.add_amount_by_line(tax),

                Type::AmountUnit => self.over_tax.add_amount_by_qty(tax),
            },

            tax_stage::Stage::OverTaxIgnorable => match tax_type {
                Type::Percentual => self.over_tax_ignorable.add_percentual(tax),

                Type::AmountLine => self.over_tax_ignorable.add_amount_by_line(tax),

                Type::AmountUnit => self.over_tax_ignorable.add_amount_by_qty(tax),
            },
        }
    }

    fn compute(&self, unit_value: BigDecimal, qty: BigDecimal) -> Result<BigDecimal, TaxError> {
        let over_taxable = self.over_taxable.tax(unit_value.clone(), qty.clone())?;
        let over_tax = self.over_tax.tax(unit_value.clone(), qty.clone())?;
        let over_tax_ignorable = self
            .over_tax_ignorable
            .tax(unit_value.clone(), qty.clone())?;

        Ok(over_taxable + over_tax + over_tax_ignorable)
    }

    fn compute_from_f64(&self, unit_value: f64, qty: f64) -> Result<BigDecimal, TaxError> {
        let over_taxable = self.over_taxable.tax_from_f64(unit_value, qty)?;
        let over_tax = self.over_tax.tax_from_f64(unit_value, qty)?;
        let over_tax_ignorable = self.over_tax_ignorable.tax_from_f64(unit_value, qty)?;

        Ok(over_taxable + over_tax + over_tax_ignorable)
    }

    fn compute_from_str<S: Into<String>>(
        &self,
        unit_value: S,
        qty: S,
    ) -> Result<BigDecimal, TaxError> {
        let opt_uv = BigDecimal::from_str(unit_value.into().as_str());

        match opt_uv {
            Ok(uv) => {
                let opt_qty = BigDecimal::from_str(qty.into().as_str());

                match opt_qty {
                    Ok(q) => self.compute(uv, q),
                    Err(e) => Err(TaxError::InvalidDecimal(format!("invalid qty {e}"))),
                }
            }
            Err(e) => Err(TaxError::InvalidDecimal(format!(
                " invalid unit value: {e}"
            ))),
        }
    }

    fn un_tax(&self, taxed: BigDecimal, qty: BigDecimal) -> Result<BigDecimal, TaxError> {
        let taxable_over_tax_ignorable = self.over_tax_ignorable.un_tax(taxed, qty.clone())?;
        let taxable_over_tax = self
            .over_tax
            .un_tax(taxable_over_tax_ignorable, qty.clone())?;
        let original_taxable = self.over_taxable.un_tax(taxable_over_tax, qty.clone())?;

        Ok(original_taxable)
    }

    fn un_tax_from_f64(&self, taxed: f64, qty: f64) -> Result<BigDecimal, TaxError> {
        let taxable_over_tax_ignorable = self
            .over_tax_ignorable
            .un_tax_from_f64(taxed, qty)?
            .to_f64()
            .unwrap();
        let taxable_over_tax = self
            .over_tax
            .un_tax_from_f64(taxable_over_tax_ignorable, qty)?
            .to_f64()
            .unwrap();
        let original_taxable = self.over_taxable.un_tax_from_f64(taxable_over_tax, qty)?;

        Ok(original_taxable)
    }

    fn un_tax_from_str<S: Into<String>>(&self, taxed: S, qty: S) -> Result<BigDecimal, TaxError> {
        let opt_taxed = BigDecimal::from_str(taxed.into().as_str());

        match opt_taxed {
            Ok(txd) => {
                let opt_qty = BigDecimal::from_str(qty.into().as_str());

                match opt_qty {
                    Ok(q) => self.un_tax(txd, q),
                    Err(e) => Err(TaxError::InvalidDecimal(format!(
                        "invalid qty in un_tax_from_str {e}"
                    ))),
                }
            }
            Err(e) => Err(TaxError::InvalidDecimal(format!(
                " invalid taxed value in un_tax_from_str: {e}"
            ))),
        }
    }
}
