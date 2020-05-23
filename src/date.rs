use crate::error::{AppError, Result};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Date {
    day: usize,
    month: usize,
    year: usize,
}

impl TryFrom<&str> for Date {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();

        if parts.len() != 3 {
            return Err(AppError::InvalidDate(format!(
                "Wrong date format: {}",
                value
            )));
        }

        let year = parts[0].parse::<usize>()?;
        let month = parts[1].parse::<usize>()?;
        let day = parts[2].parse::<usize>()?;

        Ok(Date { day, month, year })
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Date {
    pub fn validate(&self) -> Result<()> {
        if self.month == 0 || self.month > 12 {
            return Err(AppError::InvalidDate(format!(
                "Month value is invalid: {}",
                self.month
            )));
        }
        if self.day == 0 || self.day > 31 {
            return Err(AppError::InvalidDate(format!(
                "Day value is invalid: {}",
                self.day
            )));
        }
        Ok(())
    }
}
