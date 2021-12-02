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
