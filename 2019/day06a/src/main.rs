use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::io::{stdin, BufRead};
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Eq)]
struct OrbitMap(HashMap<Object, Object>);

impl OrbitMap {
    fn parse(input: impl BufRead) -> Result<Self> {
        Ok(OrbitMap(HashMap::from_iter(
            input
                .lines()
                .map(|l| {
                    l.context("could not read line")
                        .and_then(|l| Orbit::parse(&l).map(|o| (o.object, o.around)))
                })
                .collect::<Result<Vec<(Object, Object)>>>()?
                .into_iter(),
        )))
    }

    fn count_orbits(&self, object: &str) -> usize {
        self.0
            .get(object)
            .map(|around| 1 + self.count_orbits(around))
            .unwrap_or(0)
    }
}

type Object = String;

#[derive(Debug, PartialEq, Eq)]
struct Orbit {
    object: Object,
    around: Object,
}

const ORBIT_SPLIT: char = ')';

impl Orbit {
    fn parse(input: &str) -> Result<Self> {
        let mut parts = input.split(ORBIT_SPLIT);
        let around = parts
            .next()
            .ok_or_else(|| anyhow!("unable to find around"))?
            .to_string();
        let object = parts.collect::<String>();
        if object == "" {
            return Err(anyhow!("missing object"));
        }
        Ok(Orbit { object, around })
    }
}

fn main() -> Result<()> {
    let om = OrbitMap::parse(stdin().lock())?;
    println!("{}", om.0.keys().map(|k| om.count_orbits(k)).sum::<usize>());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    const TEST_INPUT: &str = include_str!("../res/test_input");

    #[test]
    fn test_orbit_parse() -> Result<()> {
        assert!(Orbit::parse("fart").is_err());
        assert_eq!(
            Orbit::parse("AAA)BBB")?,
            Orbit {
                object: "BBB".to_string(),
                around: "AAA".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn test_orbit_parse_map() -> Result<()> {
        assert!(OrbitMap::parse("fart".as_bytes()).is_err());
        let om = OrbitMap::parse(TEST_INPUT.as_bytes())?;
        assert_eq!(om.0.get("B"), Some(&"COM".to_string()));
        assert_eq!(om.0.get("L"), Some(&"K".to_string()));
        Ok(())
    }

    #[test]
    fn test_orbit_map_count_orbits() -> Result<()> {
        let om = OrbitMap::parse(TEST_INPUT.as_bytes())?;
        assert_eq!(om.count_orbits("D"), 3);
        assert_eq!(om.count_orbits("L"), 7);
        assert_eq!(om.count_orbits("COM"), 0);
        Ok(())
    }
}
