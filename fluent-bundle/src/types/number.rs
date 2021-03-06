use std::borrow::Cow;
use std::convert::TryInto;
use std::default::Default;
use std::str::FromStr;

use intl_pluralrules::operands::PluralOperands;

use crate::bundle::FluentArgs;
use crate::types::FluentValue;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum FluentNumberStyle {
    Decimal,
    Currency,
    Percent,
}

impl std::default::Default for FluentNumberStyle {
    fn default() -> Self {
        Self::Decimal
    }
}

impl From<&str> for FluentNumberStyle {
    fn from(input: &str) -> Self {
        match input {
            "decimal" => Self::Decimal,
            "currency" => Self::Currency,
            "percent" => Self::Percent,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum FluentNumberCurrencyDisplayStyle {
    Symbol,
    Code,
    Name,
}

impl std::default::Default for FluentNumberCurrencyDisplayStyle {
    fn default() -> Self {
        Self::Symbol
    }
}

impl From<&str> for FluentNumberCurrencyDisplayStyle {
    fn from(input: &str) -> Self {
        match input {
            "symbol" => Self::Symbol,
            "code" => Self::Code,
            "name" => Self::Name,
            _ => Self::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct FluentNumberOptions {
    pub style: FluentNumberStyle,
    pub currency: Option<String>,
    pub currency_display: FluentNumberCurrencyDisplayStyle,
    pub use_grouping: bool,
    pub minimum_integer_digits: Option<usize>,
    pub minimum_fraction_digits: Option<usize>,
    pub maximum_fraction_digits: Option<usize>,
    pub minimum_significant_digits: Option<usize>,
    pub maximum_significant_digits: Option<usize>,
}

impl FluentNumberOptions {
    pub fn merge(&mut self, opts: &FluentArgs) {
        for (key, value) in opts {
            match (*key, value) {
                ("style", FluentValue::String(n)) => {
                    self.style = n.as_ref().into();
                }
                ("currency", FluentValue::String(n)) => {
                    self.currency = Some(n.to_string());
                }
                ("currencyDisplay", FluentValue::String(n)) => {
                    self.currency_display = n.as_ref().into();
                }
                ("useGrouping", FluentValue::String(n)) => match n.as_ref() {
                    "true" => self.use_grouping = true,
                    "false" => self.use_grouping = false,
                    _ => {}
                },
                ("minimumIntegerDigits", FluentValue::Number(n)) => {
                    self.minimum_integer_digits = Some(n.into());
                }
                ("minimumFractionDigits", FluentValue::Number(n)) => {
                    self.minimum_fraction_digits = Some(n.into());
                }
                ("maximumFractionDigits", FluentValue::Number(n)) => {
                    self.maximum_fraction_digits = Some(n.into());
                }
                ("minimumSignificantDigits", FluentValue::Number(n)) => {
                    self.minimum_significant_digits = Some(n.into());
                }
                ("maximumSignificantDigits", FluentValue::Number(n)) => {
                    self.maximum_significant_digits = Some(n.into());
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FluentNumber {
    pub value: f64,
    pub options: FluentNumberOptions,
}

impl FluentNumber {
    pub fn new(value: f64, options: FluentNumberOptions) -> Self {
        Self { value, options }
    }

    pub fn as_string(&self) -> Cow<'static, str> {
        let mut val = self.value.to_string();
        if let Some(minfd) = self.options.minimum_fraction_digits {
            if let Some(pos) = val.find('.') {
                let frac_num = val.len() - pos - 1;
                let missing = if frac_num > minfd {
                    0
                } else {
                    minfd - frac_num
                };
                val = format!("{}{}", val, "0".repeat(missing));
            } else {
                val = format!("{}.{}", val, "0".repeat(minfd));
            }
        }
        val.into()
    }
}

impl FromStr for FluentNumber {
    type Err = std::num::ParseFloatError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        f64::from_str(input).map(|n| {
            let mfd = input.find('.').map(|pos| input.len() - pos - 1);
            let opts = FluentNumberOptions {
                minimum_fraction_digits: mfd,
                ..Default::default()
            };
            FluentNumber::new(n, opts)
        })
    }
}

impl<'l> Into<FluentValue<'l>> for FluentNumber {
    fn into(self) -> FluentValue<'l> {
        FluentValue::Number(self)
    }
}

macro_rules! from_num {
    ($num:ty) => {
        impl From<$num> for FluentNumber {
            fn from(n: $num) -> Self {
                FluentNumber {
                    value: n as f64,
                    options: FluentNumberOptions::default(),
                }
            }
        }
        impl From<&$num> for FluentNumber {
            fn from(n: &$num) -> Self {
                FluentNumber {
                    value: *n as f64,
                    options: FluentNumberOptions::default(),
                }
            }
        }
        impl Into<$num> for FluentNumber {
            fn into(self) -> $num {
                self.value as $num
            }
        }
        impl Into<$num> for &FluentNumber {
            fn into(self) -> $num {
                self.value as $num
            }
        }
        impl From<$num> for FluentValue<'_> {
            fn from(n: $num) -> Self {
                FluentValue::Number(n.into())
            }
        }
        impl From<&$num> for FluentValue<'_> {
            fn from(n: &$num) -> Self {
                FluentValue::Number(n.into())
            }
        }
    };
    ($($num:ty)+) => {
        $(from_num!($num);)+
    };
}

impl Into<PluralOperands> for &FluentNumber {
    fn into(self) -> PluralOperands {
        let mut operands: PluralOperands = self
            .value
            .try_into()
            .expect("Failed to generate operands out of FluentNumber");
        if let Some(mfd) = self.options.minimum_fraction_digits {
            if mfd > operands.v {
                operands.f *= 10_usize.pow(mfd as u32 - operands.v as u32);
                operands.v = mfd;
            }
        }
        // XXX: Add support for other options.
        operands
    }
}

from_num!(i8 i16 i32 i64 i128 isize);
from_num!(u8 u16 u32 u64 u128 usize);
from_num!(f32 f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_from_copy_ref() {
        let x = 1i16;
        let y = &x;
        let z: FluentValue = y.into();
        assert_eq!(z, FluentValue::try_number(1));
    }
}
