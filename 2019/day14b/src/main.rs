use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::io::{stdin, Read};

const ORE: &str = "ORE";
const FUEL: &str = "FUEL";

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChemicalAmount {
    name: String,
    count: usize,
}

impl ChemicalAmount {
    fn parse(input: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([0-9]+)\s+([^ ]+)$").unwrap();
        }
        let capture = RE
            .captures_iter(input)
            .next()
            .ok_or_else(|| anyhow!("could not find point"))?;
        Ok(ChemicalAmount {
            name: capture[2].to_string(),
            count: capture[1].parse()?,
        })
    }

    fn parse_many(input: &str) -> Result<Vec<Self>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r",\s*").unwrap();
        }
        RE.split(input).map(ChemicalAmount::parse).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Reaction {
    input: Vec<ChemicalAmount>,
    output: ChemicalAmount,
}

impl Reaction {
    fn parse(input: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.*?)\s*=>\s*(.*?)$").unwrap();
        }
        let capture = RE
            .captures_iter(input)
            .next()
            .ok_or_else(|| anyhow!("could not find point"))?;
        Ok(Reaction {
            input: ChemicalAmount::parse_many(&capture[1])?,
            output: ChemicalAmount::parse(&capture[2])?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Reactions(HashMap<String, Reaction>);

impl Reactions {
    fn parse(input: &str) -> Result<Self> {
        let list = input
            .trim()
            .lines()
            .map(Reaction::parse)
            .collect::<Result<Vec<Reaction>>>()?;

        let mut reactions: HashMap<String, Reaction> = HashMap::new();
        for r in list {
            reactions.insert(r.output.name.to_owned(), r);
        }
        Ok(Reactions(reactions))
    }

    fn req_ore_existing(
        &self,
        output_chem: &str,
        n: usize,
        existing: HashMap<String, usize>,
    ) -> Result<(usize, HashMap<String, usize>)> {
        let mut existing = existing;
        let have = existing.entry(output_chem.to_string()).or_default();
        let req_chem = n - usize::min(n, *have);
        *have -= n - req_chem;

        if output_chem == ORE {
            return Ok((req_chem, existing));
        }

        let reaction = self
            .0
            .get(output_chem)
            .ok_or_else(|| anyhow!("could not find reaction for {}", output_chem))?;
        let mul = (req_chem + reaction.output.count - 1) / reaction.output.count;
        let mut req_ore = 0;
        for input in &reaction.input {
            let (ore, new_existing) =
                self.req_ore_existing(&input.name, input.count * mul, existing.clone())?;
            req_ore += ore;
            existing = new_existing;
        }
        // Add any leftovers to the existing chemicals
        let chem = existing.entry(output_chem.to_string()).or_default();
        *chem += reaction.output.count * mul - req_chem;

        Ok((req_ore, existing))
    }

    fn req_ore(&self, output_chem: &str, n: usize) -> Result<(usize, HashMap<String, usize>)> {
        self.req_ore_existing(output_chem, n, HashMap::new())
    }

    fn max_fuel(&self, ore: usize) -> Result<usize> {
        // Find upper bound first
        let mut upper_fuel: usize = 1;
        loop {
            let (ore, _) = self.req_ore(FUEL, upper_fuel)?;
            if ore > MAX_ORE {
                break;
            }
            upper_fuel *= 2;
        }
        let mut lower_fuel: usize = upper_fuel / 2;
        while lower_fuel < upper_fuel {
            let mid = (lower_fuel + upper_fuel) / 2 + 1;
            let (req_ore, _) = self.req_ore(FUEL, mid)?;
            if req_ore > ore {
                upper_fuel = mid - 1;
            } else {
                lower_fuel = mid;
            }
        }
        Ok(lower_fuel)
    }
}

const MAX_ORE: usize = 1_000_000_000_000;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input)?;
    let reactions = Reactions::parse(&input)?;
    println!("{}", reactions.max_fuel(MAX_ORE)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chemical_amount_parse() -> Result<()> {
        assert_eq!(
            ChemicalAmount::parse("12 HKGWZ")?,
            ChemicalAmount {
                name: "HKGWZ".to_string(),
                count: 12,
            }
        );
        Ok(())
    }

    #[test]
    fn test_chemical_amount_parse_many() -> Result<()> {
        assert_eq!(
            ChemicalAmount::parse_many("3 NPRST, 1 KGSDJ, 1 CTVK")?,
            vec![
                ChemicalAmount {
                    name: "NPRST".to_string(),
                    count: 3,
                },
                ChemicalAmount {
                    name: "KGSDJ".to_string(),
                    count: 1,
                },
                ChemicalAmount {
                    name: "CTVK".to_string(),
                    count: 1,
                },
            ]
        );
        Ok(())
    }

    #[test]
    fn test_reaction_parse() -> Result<()> {
        assert_eq!(
            Reaction::parse("3 NPRST, 1 KGSDJ, 1 CTVK => 2 QMBM")?,
            Reaction {
                input: vec![
                    ChemicalAmount {
                        name: "NPRST".to_string(),
                        count: 3,
                    },
                    ChemicalAmount {
                        name: "KGSDJ".to_string(),
                        count: 1,
                    },
                    ChemicalAmount {
                        name: "CTVK".to_string(),
                        count: 1,
                    },
                ],
                output: ChemicalAmount {
                    name: "QMBM".to_string(),
                    count: 2,
                },
            },
        );
        Ok(())
    }

    #[test]
    fn test_reactions_parse() -> Result<()> {
        assert_eq!(
            Reactions::parse(
                "
3 NPRST, 1 KGSDJ, 1 CTVK => 2 QMBM
7 VJMWM => 4 JHDW
"
            )?,
            Reactions(
                vec![
                    (
                        "QMBM".to_string(),
                        Reaction {
                            input: vec![
                                ChemicalAmount {
                                    name: "NPRST".to_string(),
                                    count: 3,
                                },
                                ChemicalAmount {
                                    name: "KGSDJ".to_string(),
                                    count: 1,
                                },
                                ChemicalAmount {
                                    name: "CTVK".to_string(),
                                    count: 1,
                                },
                            ],
                            output: ChemicalAmount {
                                name: "QMBM".to_string(),
                                count: 2,
                            },
                        }
                    ),
                    (
                        "JHDW".to_string(),
                        Reaction {
                            input: vec![ChemicalAmount {
                                name: "VJMWM".to_string(),
                                count: 7,
                            },],
                            output: ChemicalAmount {
                                name: "JHDW".to_string(),
                                count: 4,
                            },
                        }
                    ),
                ]
                .into_iter()
                .collect()
            ),
        );
        Ok(())
    }

    #[test]
    fn test_reactions_simple() -> Result<()> {
        let reactions = Reactions::parse(
            "
9 ORE => 2 A
3 A => 1 FUEL
        ",
        )?;
        assert_eq!(reactions.req_ore("FUEL", 1)?.0, 18);
        Ok(())
    }

    #[test]
    fn test_reactions_composite() -> Result<()> {
        let reactions = Reactions::parse(
            "
9 ORE => 2 A
8 ORE => 3 B
3 A, 3 B => 1 FUEL
        ",
        )?;
        assert_eq!(reactions.req_ore("FUEL", 1)?.0, 26);
        Ok(())
    }

    #[test]
    fn test_reactions_leftover() -> Result<()> {
        let reactions = Reactions::parse(
            "
9 ORE => 2 A
1 A => 3 B
3 A, 3 B => 1 FUEL
        ",
        )?;
        assert_eq!(reactions.req_ore("FUEL", 1)?.0, 18);
        Ok(())
    }

    #[test]
    fn test_reactions_1() -> Result<()> {
        let reactions = Reactions::parse(
            "
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
        ",
        )?;
        assert_eq!(reactions.req_ore(FUEL, 1)?.0, 165);
        Ok(())
    }

    #[test]
    fn test_reactions_2() -> Result<()> {
        let reactions = Reactions::parse(
            "
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
        ",
        )?;
        assert_eq!(reactions.req_ore(FUEL, 1)?.0, 13312);
        assert_eq!(reactions.max_fuel(MAX_ORE)?, 82892753);
        Ok(())
    }

    #[test]
    fn test_reactions_3() -> Result<()> {
        let reactions = Reactions::parse(
            "
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
        ",
        )?;
        assert_eq!(reactions.req_ore(FUEL, 1)?.0, 180697);
        assert_eq!(reactions.max_fuel(MAX_ORE)?, 5586022);
        Ok(())
    }

    #[test]
    fn test_reactions_4() -> Result<()> {
        let reactions = Reactions::parse(
            "
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
        ",
        )?;
        assert_eq!(reactions.req_ore(FUEL, 1)?.0, 2210736);
        assert_eq!(reactions.max_fuel(MAX_ORE)?, 460664);
        Ok(())
    }
}
