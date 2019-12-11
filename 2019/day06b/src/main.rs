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

    fn orbits(&self, object: &str) -> Vec<Object> {
        match self.0.get(object) {
            Some(parent) => {
                let mut parent_orbits = self.orbits(parent);
                parent_orbits.push(parent.to_owned());
                parent_orbits
            }
            None => vec![],
        }
    }

    fn orbital_transfers_between(&self, a: &str, b: &str) -> usize {
        let a_orbits = self.orbits(a);
        let b_orbits = self.orbits(b);
        let shortest = std::cmp::min(a_orbits.len(), b_orbits.len());
        let mut shared = 0;
        for i in 0..shortest {
            if a_orbits[i] != b_orbits[i] {
                break;
            }
            shared += 1;
        }
        a_orbits.len() + b_orbits.len() - shared * 2
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
    println!("{}", om.orbital_transfers_between("SAN", "YOU"));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    const TEST_INPUT: &str = include_str!("../res/test_input");
    const TEST_INPUT_B: &str = include_str!("../res/test_input_b");

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
    fn test_orbit_map_orbital_transfers_between() -> Result<()> {
        let om = OrbitMap::parse(TEST_INPUT_B.as_bytes())?;
        assert_eq!(om.orbital_transfers_between("SAN", "YOU"), 4);
        Ok(())
    }
}
