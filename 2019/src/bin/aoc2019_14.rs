use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, Read};


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Chemical (String);

impl Chemical {
    fn new(name: &str) -> Self {
        Chemical(name.to_string())
    }
}


#[derive(Clone, Debug, PartialEq)]
struct ChemicalAmount {
    amount: u32,
    chemical: Chemical,
}

impl ChemicalAmount {
    fn new(amount: u32, chemical: Chemical) -> Self {
        Self { amount, chemical }
    }

    fn from_string(input: &str) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();
        if parts.len() != 2 { panic!("Unexpected chemical/amount split: {} parts", parts.len()); }
        Self {
            amount: parts[0].parse::<u32>().expect(&format!("Could not parse amount: {}", parts[0])),
            chemical: Chemical::new(parts[1])
        }
    }
}


#[derive(Debug)]
struct Reaction {
    result: ChemicalAmount,
    requirements: Vec<ChemicalAmount>,
}

impl Reaction {
    fn from_string(input: &str) -> Reaction {
        let parts: Vec<&str> = input.trim().split(" => ").collect();
        if parts.len() != 2 { panic!("Unexpected reaction split: {} parts", parts.len()); }
        let requirements: Vec<&str> = parts[0].split(", ").collect();
        Self {
            result: ChemicalAmount::from_string(parts[1]),
            requirements: requirements.iter().map(|&string| ChemicalAmount::from_string(&string)).collect(),
        }
    }
}


#[derive(Debug)]
struct ReactionTable {
    reactions: HashMap<Chemical, Reaction>,
}

impl ReactionTable {
    fn new() -> Self {
        Self { reactions: HashMap::new() }
    }

    fn from_string(input: &str) -> Self {
        let mut table = Self::new();
        for line in input.trim().lines() {
            let reaction = Reaction::from_string(&line);
            match table.reactions.insert(reaction.result.chemical.clone(), reaction) {
                Some(previous) => panic!("There are multiple reactions for {:?}", previous.result.chemical),
                _ => ()
            }
        }
        table
    }

    fn direct_requirements_and_leftovers(&self, result: ChemicalAmount) -> Option<(Vec<ChemicalAmount>, ChemicalAmount)> {
        match self.reactions.get(&result.chemical) {
            None => None,
            Some(reaction) => {
                let factor = ((result.amount as f32) / (reaction.result.amount as f32)).ceil() as u32;
                let leftover = (factor * reaction.result.amount) - result.amount;
                if leftover >= reaction.result.amount {
                    panic!("For {:?}, {} units were requested, factor {} was determined, {} is left over.", result.chemical, result.amount, factor, leftover);
                }
                Some((
                    reaction.requirements.iter().map(|requirement| ChemicalAmount::new(requirement.amount * factor, requirement.chemical.clone())).collect(),
                    ChemicalAmount::new(leftover, result.chemical.clone()),
                ))
            },
        }
    }

    fn requirements(&self, result: ChemicalAmount) -> Vec<ChemicalAmount> {
        let mut by_chemical: HashMap<Chemical, i32> = HashMap::new();
        by_chemical.insert(result.chemical.clone(), result.amount.try_into().unwrap());
        let mut change: Option<(Vec<ChemicalAmount>, ChemicalAmount)>;
        loop {
            change = None;
            for (chemical, amount) in by_chemical.iter() {
                if *amount <= 0 { continue; }
                match self.direct_requirements_and_leftovers(ChemicalAmount::new((*amount).try_into().unwrap(), chemical.clone())) {
                    None => (),
                    Some(tuple) => { change = Some(tuple); break; }
                }
            }
            //println!("Change: {:?}", change.clone().unwrap());
            if change == None { break; }
            let (requirements, leftover) = change.unwrap();
            by_chemical.remove(&leftover.chemical);
            for requirement in requirements.iter() {
                by_chemical.entry(requirement.clone().chemical)
                    .and_modify(|entry| *entry += requirement.amount as i32)
                    .or_insert(requirement.amount.try_into().unwrap());
            }
            by_chemical.entry(leftover.clone().chemical)
                .and_modify(|entry| *entry -= leftover.amount as i32)
                .or_insert(0 - leftover.amount as i32);
            by_chemical.retain(|_, amount| *amount != 0);
            //println!("After step: {:?}", by_chemical);
        }
        by_chemical.iter().filter(|(_, &amount)| amount >= 0).map(|(chemical, amount)| ChemicalAmount::new((*amount).try_into().unwrap(), chemical.clone())).collect()
    }
}


fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let table = ReactionTable::from_string(&buffer);

    println!("{:?}", table.requirements(ChemicalAmount::from_string("1 FUEL")));
    //println!("You need {} ORE for 1 FUEL.", table.requirements(ChemicalAmount::from_string("1 FUEL"))[0].amount);
}


#[test]
fn example_a1() {
    let table = ReactionTable::from_string("
        10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL
    ");
    assert_eq!(table.requirements(ChemicalAmount::from_string("1 B")), vec![ChemicalAmount::from_string("1 ORE")]);
    assert_eq!(table.requirements(ChemicalAmount::from_string("7 A")), vec![ChemicalAmount::from_string("10 ORE")]);
    assert_eq!(table.requirements(ChemicalAmount::from_string("1 C")), vec![ChemicalAmount::from_string("11 ORE")]);
    assert_eq!(table.requirements(ChemicalAmount::from_string("1 D")), vec![ChemicalAmount::from_string("21 ORE")]);
    assert_eq!(table.requirements(ChemicalAmount::from_string("1 E")), vec![ChemicalAmount::from_string("31 ORE")]);
    assert_eq!(table.requirements(ChemicalAmount::from_string("1 FUEL")), vec![ChemicalAmount::from_string("31 ORE")]);
}

#[test]
fn example_a2() {
    let table = ReactionTable::from_string("
        9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL
    ");
    assert_eq!(table.requirements(ChemicalAmount::from_string("1 FUEL")), vec![ChemicalAmount::from_string("165 ORE")]);
}

#[test]
fn example_a3() {
    let table = ReactionTable::from_string("
        157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
    ");
    assert_eq!(table.requirements(ChemicalAmount::from_string("1 FUEL")), vec![ChemicalAmount::from_string("13312 ORE")]);
}

#[test]
fn example_a4() {
    let table = ReactionTable::from_string("
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
    ");
    assert_eq!(table.requirements(ChemicalAmount::from_string("1 FUEL")), vec![ChemicalAmount::from_string("180697 ORE")]);
}

#[test]
fn example_a5() {
    let table = ReactionTable::from_string("
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
    ");
    assert_eq!(table.requirements(ChemicalAmount::from_string("1 FUEL")), vec![ChemicalAmount::from_string("2210736 ORE")]);
}

#[test]
fn test_reddit_example() {
    let table = ReactionTable::from_string("
        1 ORE => 2 A
        1 A => 1 B
        1 A, 1 B => 1 FUEL
    ");
    assert_eq!(table.requirements(ChemicalAmount::from_string("1 FUEL")), vec![ChemicalAmount::from_string("1 ORE")]);
}
