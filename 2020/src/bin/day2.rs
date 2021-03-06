use std::str::FromStr;

use anyhow::Result;
use structopt::StructOpt;
use thiserror::Error;

use aoc_2020::{read_lines, Args};

#[derive(Debug)]
struct Policy {
    lower_count: usize,
    higher_count: usize,
    letter: char,
}

#[derive(Debug, Error)]
enum PolicyError {
    #[error("general error: {0}")]
    General(String),
}

impl FromStr for Policy {
    type Err = PolicyError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split("-").collect();
        let lower_count: usize = parts
            .get(0)
            .ok_or_else(|| PolicyError::General(format!("{} missing lower", line)))?
            .parse()
            .map_err(|_| PolicyError::General("can't parse".to_string()))?;
        let parts: Vec<&str> = parts
            .get(1)
            .ok_or_else(|| PolicyError::General(format!("{} missing rest", line)))?
            .split(" ")
            .collect();
        let higher_count: usize = parts
            .get(0)
            .ok_or_else(|| PolicyError::General(format!("{} missing higher", line)))?
            .parse()
            .map_err(|_| PolicyError::General("can't parse".to_string()))?;
        let parts: Vec<&str> = parts
            .get(1)
            .ok_or_else(|| PolicyError::General(format!("{} missing rest", line)))?
            .split(":")
            .collect();
        let letter: char = parts
            .get(0)
            .ok_or_else(|| PolicyError::General(format!("{} missing letter", line)))?
            .parse()
            .map_err(|_| PolicyError::General("can't parse".to_string()))?;
        Ok(Self {
            lower_count,
            higher_count,
            letter,
        })
    }
}

#[derive(Debug)]
struct Password {
    policy: Policy,
    password: String,
}

#[derive(Debug, Error)]
enum PasswordError {
    #[error("policy error")]
    Policy(#[from] PolicyError),
    #[error("general error: {0}")]
    General(String),
}

fn str_has_char_at(haystack: &str, needle: char, index: usize) -> bool {
    if let Some(letter) = haystack.chars().nth(index - 1) {
        if letter == needle {
            return true;
        }
    }

    false
}

impl FromStr for Password {
    type Err = PasswordError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let policy = Policy::from_str(line)?;

        let parts: Vec<&str> = line.split(" ").collect();
        let password = parts
            .get(2)
            .ok_or_else(|| PasswordError::General(format!("{} missing password", line)))?
            .to_string();

        Ok(Self { policy, password })
    }
}

impl Password {
    fn passes(&self) -> bool {
        let first_okay =
            str_has_char_at(&self.password, self.policy.letter, self.policy.lower_count);
        let second_okay =
            str_has_char_at(&self.password, self.policy.letter, self.policy.higher_count);

        first_okay ^ second_okay
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let passwords: Vec<Password> = read_lines(&args.filename)?;

    let mut count = 0;
    for password in passwords.into_iter() {
        if password.passes() {
            count += 1;
        }
    }

    println!("{}", count);

    Ok(())
}
