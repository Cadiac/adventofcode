use std::str::FromStr;
use std::fmt::Debug;

pub fn parse_from_str<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|depth| depth.parse::<T>().unwrap())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coords<T> {
    pub x: T,
    pub y: T,
}
