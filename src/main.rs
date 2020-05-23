mod date;
mod error;
mod storage;

use date::Date;
use error::AppError;
use std::{convert::TryFrom, io::Write};
use storage::{ExecutionStatus, Storage};

fn main() -> error::Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut input_buf = String::new();
    let mut storage = Storage::new();

    writeln!(stdout, "~ Database CLI ~\r")?;
    stdout.flush()?;
    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;

        input_buf.clear();
        stdin.read_line(&mut input_buf)?;

        let command = Command::try_from(input_buf.split_ascii_whitespace().collect::<Vec<_>>())?;
        match storage.execute(command) {
            ExecutionStatus::Ok => (),
            ExecutionStatus::NoDate => writeln!(stdout, "> Date not found")?,
            ExecutionStatus::NoEvent => writeln!(stdout, "> Event not found")?,
            ExecutionStatus::Deleted(count) => {
                if let Some(num) = count {
                    writeln!(stdout, "> Deleted {} events", num)?;
                } else {
                    writeln!(stdout, "> Deleted successfully")?;
                }
            }
            ExecutionStatus::Found(events) => {
                for str in events {
                    writeln!(stdout, "{}", str)?;
                }
            }
            ExecutionStatus::Printed(table) => {
                for (date, events) in table {
                    for str in events {
                        writeln!(stdout, "{} {}", date, str)?;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Command<'a> {
    Add(Date, &'a str),
    Delete(Date, Option<&'a str>),
    Find(Date),
    Print,
}

impl<'a> TryFrom<Vec<&'a str>> for Command<'a> {
    type Error = AppError;

    fn try_from(value: Vec<&'a str>) -> Result<Self, Self::Error> {
        match value[0] {
            "add" => {
                let date = Date::try_from(value[1])?;
                date.validate()?;
                Ok(Command::Add(date, value[2]))
            }
            "del" => {
                let date = Date::try_from(value[1])?;
                date.validate()?;
                if value.len() == 3 {
                    Ok(Command::Delete(date, Some(value[2])))
                } else {
                    Ok(Command::Delete(date, None))
                }
            }
            "find" => {
                let date = Date::try_from(value[1])?;
                date.validate()?;
                Ok(Command::Find(date))
            }
            "print" => Ok(Command::Print),
            "exit" => std::process::exit(0),
            _ => Err(AppError::InvalidCommand(value[0].to_string())),
        }
    }
}
