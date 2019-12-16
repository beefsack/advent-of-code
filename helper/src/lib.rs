use anyhow::Result;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub mod point;

pub fn parse_lines<T, R>(input: R) -> impl Iterator<Item = Result<T>>
where
    R: Read,
    T: FromStr,
    T::Err: 'static + Error + Send + Sync,
{
    BufReader::new(input).lines().map(|l| parse_line(l?))
}

pub fn parse_line<T>(line: String) -> Result<T>
where
    T: FromStr,
    T::Err: 'static + Error + Send + Sync,
{
    Ok(line.parse()?)
}
