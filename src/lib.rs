// Copyright 2023 Andrés Reyes El Programador Pobre.
// 
// Licenced under the MIT License
// 
// Por la presente se concede permiso, libre de cargos, a cualquier persona que 
// obtenga una copia de este software y de los archivos de documentación asociados (el "Software"), 
// a utilizar el Software sin restricción, incluyendo sin limitación los derechos a usar, 
// copiar, modificar, fusionar, publicar, distribuir, sublicenciar, y/o vender copias del Software, 
// y a permitir a las personas a las que se les proporcione el Software
// a hacer lo mismo, sujeto a las siguientes condiciones:
// 
// El aviso de copyright anterior y este aviso de permiso se incluirán en todas las copias o partes sustanciales del Software.
// EL SOFTWARE SE PROPORCIONA "COMO ESTÁ", SIN GARANTÍA DE NINGÚN TIPO, EXPRESA O IMPLÍCITA, INCLUYENDO PERO NO LIMITADO A GARANTÍAS DE COMERCIALIZACIÓN, IDONEIDAD PARA UN PROPÓSITO PARTICULAR E INCUMPLIMIENTO. EN NINGÚN CASO LOS AUTORES O PROPIETARIOS DE LOS DERECHOS DE AUTOR SERÁN RESPONSABLES DE NINGUNA RECLAMACIÓN, DAÑOS U OTRAS RESPONSABILIDADES, YA SEA EN UNA ACCIÓN DE CONTRATO, AGRAVIO O CUALQUIER OTRO MOTIVO, DERIVADAS DE, FUERA DE O EN CONEXIÓN CON EL SOFTWARE O SU USO U OTRO TIPO DE ACCIONES EN EL SOFTWARE.
//
//! calculus
//! 
//! `calculus` provee una serie de utilidades para calcular eficientemente subtotales de lineas de detalle
//! 
use std::{fmt, str::FromStr};

use bigdecimal::{BigDecimal, FromPrimitive};
use discount::DiscountComputer;
use tax::{tax_stage, TaxComputer};

pub mod discount;
pub mod tax;

#[derive(Debug)] // Allow the use of "{:?}" format specifier
pub enum CalculusError {
    NegativeUnitValue(String),
    NegativeResult(String),
    NegativeQty(String),
    InvalidDecimalValue(String),
    Other(String),
}

// Allow the use of "{}" format specifier
impl fmt::Display for CalculusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculusError::NegativeUnitValue(ref msg) => write!(
                f,
                "Negative unit value Error: {msg} unit value cannot be negative"
            ),
            CalculusError::NegativeQty(ref value) => {
                write!(f, "Negative quantity value Error: {value}")
            }
            CalculusError::InvalidDecimalValue(msg) => write!(f, " Invalid decimal value {msg}"),
            CalculusError::Other(msg) => write!(f, " Error {msg}"),
            CalculusError::NegativeResult(msg) => {
                write!(f, "Negative result value Error: {msg}")
            }
        }
    }
}

pub fn hundred() -> BigDecimal {
    BigDecimal::from_str("100.0").unwrap()
}

pub fn one() -> BigDecimal {
    BigDecimal::from_str("1.0").unwrap()
}

pub fn inverse() -> BigDecimal {
    BigDecimal::from_str("-1.0").unwrap()
}

#[derive(Debug)]
pub struct Calculation {
    pub net: BigDecimal,
    pub brute: BigDecimal,
    pub tax: BigDecimal,
    pub discount: BigDecimal,
    pub net_wout_disc: BigDecimal,
    pub brute_wout_disc: BigDecimal,
    pub tax_wout_disc: BigDecimal,
    pub unit_value: BigDecimal,
    pub total_discount_percent: BigDecimal,
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

#[derive(Debug)]
pub struct CalculationF64 {
    pub net: f64,
    pub brute: f64,
    pub tax: f64,
    pub discount: f64,
    pub net_wout_disc: f64,
    pub brute_wout_disc: f64,
    pub tax_wout_disc: f64,
    pub unit_value: BigDecimal,
    pub total_discount_percent: BigDecimal,
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

pub trait Calculator {
     
    fn add_discount(
       &mut self,
       discount: BigDecimal,
       discount_type: discount::Type,
   ) -> Option<discount::DiscountError>;

    fn add_f64_discount(
       &mut self,
       discount: f64,
       discount_type: discount::Type,
   ) -> Option<discount::DiscountError>;

    fn add_str_discount<S: Into<String>>(
       &mut self,
       discount: S,
       discount_type: discount::Type,
   ) -> Option<discount::DiscountError>;

    fn add_f64_tax(
       &mut self,
       tax: f64,
       stage: tax_stage::Stage,
       tax_type: tax::Type,
   ) -> Option<tax::TaxError>;

    fn add_tax(
       &mut self,
       tax: BigDecimal,
       stage: tax_stage::Stage,
       tax_type: tax::Type,
   ) -> Option<tax::TaxError>;

    fn add_str_tax<S: Into<String>>(
       &mut self,
       tax: S,
       stage: tax_stage::Stage,
       tax_type: tax::Type,
   ) -> Option<tax::TaxError>;

    fn compute_from_brute(&self, brute: BigDecimal, qty: BigDecimal, scale: i8) -> Result<Calculation, CalculusError>;

    fn compute_from_f64_brute(&self, brute: f64, qty: f64, scale: i8) -> Result<Calculation, CalculusError>;

    fn compute_from_str_brute<S: Into<String>>(&self, brute: S, qty: S, scale: i8) -> Result<Calculation, CalculusError>;

    fn compute_from_str<S: Into<String>>(
       &self,
       unit_value: S,
       qty: S,
       scale: i8,
   ) -> Result<Calculation, CalculusError>;


    fn compute_from_f64(
       &self,
       unit_value: f64,
       qty: f64,
       scale: i8,
   ) -> Result<Calculation, CalculusError>;

    fn compute(
       &self,
       unit_value: BigDecimal,
       qty: BigDecimal,
       scale: i8,
   ) -> Result<Calculation, CalculusError>;
}


/// Able to calculate detail lines
///
/// # Example
///
/// ```
/// use std::str::FromStr;
///
/// use bigdecimal::BigDecimal;
/// use calculus::{discount, tax, Calculator};
///
/// let mut c = calculus::DetailCalculator::new();
///
/// let err = c.add_str_discount("10.0", discount::Type::Percentual);
/// assert!(err.is_none(), "error adding percentual discount");
///
/// let err = c.add_str_discount("1.0", discount::Type::AmountUnit);
/// assert!(err.is_none(), "error adding amount unit discount");
///
///
/// let err = c.add_str_tax(
///     "16.0",
///     tax::tax_stage::Stage::OverTaxable,
///     tax::Type::Percentual,
/// );
/// assert!(err.is_some(), "error adding percentual 16% tax");
///
/// let err = c.add_str_tax(
///     "1.0",
///     tax::tax_stage::Stage::OverTaxable,
///     tax::Type::AmountUnit,
/// );
/// assert!(err.is_some(), "error adding percentual 1 amount unit tax");
///
/// let r = c.compute(
///     BigDecimal::from_str("100.0").unwrap(),
///     BigDecimal::from_str("2.0").unwrap(),
///     16,
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

     fn add_f64_discount(
        &mut self,
        discount: f64,
        discount_type: discount::Type,
    ) -> Option<discount::DiscountError> {
        self.discount_handler
            .add_f64_discount(discount, discount_type)
    }

     fn add_str_discount<S: Into<String>>(
        &mut self,
        discount: S,
        discount_type: discount::Type,
    ) -> Option<discount::DiscountError> {
        self.discount_handler
            .add_str_discount(discount, discount_type)
    }

     fn add_f64_tax(
        &mut self,
        tax: f64,
        stage: tax_stage::Stage,
        tax_type: tax::Type,
    ) -> Option<tax::TaxError> {
        self.tax_handler.add_f64_tax(tax, stage, tax_type)
    }

     fn add_tax(
        &mut self,
        tax: BigDecimal,
        stage: tax_stage::Stage,
        tax_type: tax::Type,
    ) -> Option<tax::TaxError> {
        self.tax_handler.add_tax(tax, stage, tax_type)
    }

     fn add_str_tax<S: Into<String>>(
        &mut self,
        tax: S,
        stage: tax_stage::Stage,
        tax_type: tax::Type,
    ) -> Option<tax::TaxError> {
        self.tax_handler.add_str_tax(tax, stage, tax_type)
    }

     fn compute_from_brute(&self, brute: BigDecimal, qty: BigDecimal, scale: i8) -> Result<Calculation, CalculusError> {
        let scale = if scale < 0 { 16 } else { scale };

        let r_opt = self.tax_handler.un_tax(brute.clone(), qty.clone());

        match r_opt {
            Ok(net) => {
                let opt_uv = self.discount_handler.un_discount(net, qty.clone());

                match opt_uv {
                    Ok(uv) => self.compute(uv, qty, scale),
                    Err(e) => Err(CalculusError::Other(e.to_string())),
                }
                
            },
            Err(e) => Err(CalculusError::Other(e.to_string())),
        }
    }

     fn compute_from_f64_brute(&self, brute: f64, qty: f64, scale: i8) -> Result<Calculation, CalculusError> {
        let opt_bt = BigDecimal::from_f64(brute);

        match opt_bt {
            Some(bt) => {
                let opt_q = BigDecimal::from_f64(qty);

                match opt_q {
                    Some(qty) => self.compute_from_brute(bt, qty, scale),
                    None => Err(CalculusError::InvalidDecimalValue(format!("{qty}"))),
                }
            },
            None => Err(CalculusError::InvalidDecimalValue(format!("{brute}"))),
        }
    }

     fn compute_from_str_brute<S: Into<String>>(&self, brute: S, qty: S, scale: i8) -> Result<Calculation, CalculusError> {
        let opt_bt = BigDecimal::from_str(&brute.into());

        match opt_bt {
            Ok(bt) => {
                let opt_q = BigDecimal::from_str(&qty.into());

                match opt_q {
                    Ok(qty) => self.compute_from_brute(bt, qty, scale)
,
                    Err(e) => Err(CalculusError::InvalidDecimalValue(format!("{e}"))),
                }
            },
            Err(e) => Err(CalculusError::InvalidDecimalValue(format!("{e}"))),
        }
    }

     fn compute_from_str<S: Into<String>>(
        &self,
        unit_value: S,
        qty: S,
        scale: i8,
    ) -> Result<Calculation, CalculusError> {

        let opt_uv = BigDecimal::from_str(unit_value.into().clone().as_str());

        match opt_uv {
            Ok(uv) => {
                let opt_q = BigDecimal::from_str(qty.into().clone().as_str());

                match opt_q {
                    Ok(q) => self.compute(uv.clone(), q.clone(), scale),
                    Err(e) => Err(CalculusError::InvalidDecimalValue(format!("qty {e}"))),
                }
            },
            Err(e) => Err(CalculusError::InvalidDecimalValue(format!("{e} unit value"))),
        }
    }


     fn compute_from_f64(
        &self,
        unit_value: f64,
        qty: f64,
        scale: i8,
    ) -> Result<Calculation, CalculusError> {

        let opt_uv = BigDecimal::from_f64(unit_value);

        match opt_uv {
            Some(uv) => {
                let opt_q = BigDecimal::from_f64(qty);

                match opt_q {
                    Some(qty) => self.compute(uv, qty, scale),
                    None => Err(CalculusError::InvalidDecimalValue(format!("{qty}"))),
                }
            },
            None => Err(CalculusError::InvalidDecimalValue(format!("{unit_value}"))),
        }
    }

     fn compute(
        &self,
        unit_value: BigDecimal,
        qty: BigDecimal,
        scale: i8,
    ) -> Result<Calculation, CalculusError> {
        let scale = if scale < 0 { 16 } else { scale };

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

                                Ok(calc)
                            },

                            Err(e) => {
                                Err(CalculusError::Other(format!(
                                    "error calculating tax without discount {e} from unit_value {}, qty {}  scale {scale}",
                                    discounted_uv, qty
                                )))
                            },
                        }
                    },
                    Err(e) => {
                        Err(CalculusError::Other(format!(
                            "error calculating tax {e} from discounted_unit_value {}, qty {}  scale {scale}",
                            discounted_uv, qty
                        )))
                    },
                }
            }

            Err(e) => Err(CalculusError::Other(format!(
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
