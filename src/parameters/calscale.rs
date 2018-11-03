use super::super::values::Value;
use super::super::values::calscale_value::CalscaleValue;
use super::Parameter;

use std::fmt::{self, Display, Formatter};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq)]
pub struct Calscale {
    calscale_value: CalscaleValue,
}

impl Calscale {
    pub fn with_calscale_value(calscale_value: CalscaleValue) -> Calscale {
        Calscale { calscale_value }
    }
}

impl Calscale {
    pub fn get_value_type(&self) -> &CalscaleValue {
        &self.calscale_value
    }
}

impl Parameter for Calscale {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";CALSCALE=")?;

        Value::fmt(&self.calscale_value, f)?;

        Ok(())
    }
}

impl Display for Calscale {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for Calscale {}

impl ValidatedWrapper for Calscale {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
