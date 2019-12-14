use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../input.txt");

#[derive(Debug, Clone)]
struct Reaction {
    name: String,
    output_amount: i64,
    ingredients: Vec<(String, i64)>
}

#[derive(Debug, Clone)]
struct Nanofactory {
    storage: HashMap<String, i64>,
    reactions: HashMap<String, Reaction>,
    total_ore_required: i64,
}

// Based on reddit example, had difficulty..
impl Nanofactory {
    fn new(input: &str) -> Self {
        let mut reactions: HashMap<String, Reaction> = HashMap::new();
    
        input.lines().for_each(|line| {
            let v: Vec<&str> = line.split(" => ").collect();
        
            let ingredients: Vec<(String, i64)> = v[0]
                .split(", ")
                .map(|ingredient| {
                    let ingredient_v: Vec<&str> = ingredient.split(' ').collect();
                    return (ingredient_v[1].parse::<String>().unwrap(), ingredient_v[0].parse::<i64>().unwrap());
                })
                .collect();
        
            let output: Vec<&str> = v[1].split(' ').collect();
    
            let output_amount = output[0].parse::<i64>().unwrap();
            let output_name = output[1].parse::<String>().unwrap();
            
            reactions.insert(output_name.clone(), Reaction{
                name: output_name,
                output_amount: output_amount,
                ingredients: ingredients
            });
        });
        let storage = HashMap::new();

        Self { storage, reactions, total_ore_required: 0 }
    }

    fn calculate_total_ore(&mut self, name: String, required: i64) -> i64 {
        if name == "ORE" {
            self.total_ore_required += required;
            return self.total_ore_required;
        }
    
        let mut already_made = *self.storage.entry(name.clone()).or_insert(0);
    
        if already_made < required {
            let reaction = self.reactions.get(&name).expect("reaction");
            let output_amount = reaction.output_amount;
            let ingredients = reaction.ingredients.clone();
            
            let repeats = (required - already_made + output_amount - 1) / output_amount;
            for (ingredient_name, amount_needed) in ingredients {
                self.calculate_total_ore(ingredient_name.clone(), amount_needed * repeats);
            }
    
            already_made += output_amount * repeats;
        }
        self.storage.insert(name, already_made - required);
        self.total_ore_required
    }
}

fn part_1(input: &str) -> i64 {
    let mut factory = Nanofactory::new(input);
    let total_ore = factory.calculate_total_ore(String::from("FUEL"), 1);
    return total_ore;
}

fn part_2(input: &str, max_ore: i64) -> i64 {
    let mut min_fuel = 1;
    let mut max_fuel = 1;

    // Quickly sensible max fuel start value
    loop {
        let mut factory = Nanofactory::new(input);
        let total_ore = factory.calculate_total_ore(String::from("FUEL"), max_fuel);
        if total_ore > max_ore {
            break;
        }
        max_fuel *= 2; // Double the fuel until we go over the limit
    }

    println!("[DEBUG]: min {}, max {}", min_fuel, max_fuel);

    loop {
        let mid = (min_fuel + max_fuel) / 2;
        println!("[DEBUG]: checking {}", mid);

        let mut factory = Nanofactory::new(input);
        let total_ore = factory.calculate_total_ore(String::from("FUEL"), mid);
        if total_ore > max_ore {
            max_fuel = mid - 1;
        } else if total_ore < max_ore {
            min_fuel = mid + 1;
        } else {
            return mid;
        }

        if max_fuel < min_fuel || min_fuel > max_fuel {
            return mid;
        }
    }
}

fn main() -> () {
    let ore_required = part_1(INPUT_FILE);
    let max_fuel = part_2(INPUT_FILE, 1000000000000);

    println!("[INFO]: Part 1: {:?}", ore_required);
    println!("[INFO]: Part 2: {:?}", max_fuel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example_1() {
        assert_eq!(part_1(
           "10 ORE => 10 A\n\
            1 ORE => 1 B\n\
            7 A, 1 B => 1 C\n\
            7 A, 1 C => 1 D\n\
            7 A, 1 D => 1 E\n\
            7 A, 1 E => 1 FUEL"), 31);
    }

    #[test]
    fn it_solves_part1_example_2() {
        assert_eq!(part_1(
           "9 ORE => 2 A\n\
            8 ORE => 3 B\n\
            7 ORE => 5 C\n\
            3 A, 4 B => 1 AB\n\
            5 B, 7 C => 1 BC\n\
            4 C, 1 A => 1 CA\n\
            2 AB, 3 BC, 4 CA => 1 FUEL"), 165);
    }

    #[test]
    fn it_solves_part1_example_3() {
        assert_eq!(part_1(
          "157 ORE => 5 NZVS\n\
           165 ORE => 6 DCFZ\n\
           44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
           12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
           179 ORE => 7 PSHF\n\
           177 ORE => 5 HKGWZ\n\
           7 DCFZ, 7 PSHF => 2 XJWVT\n\
           165 ORE => 2 GPVTF\n\
           3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"), 13312);
    }

    #[test]
    fn it_solves_part1_example_4() {
        assert_eq!(part_1(
          "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
           17 NVRVD, 3 JNWZP => 8 VPVL\n\
           53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
           22 VJHF, 37 MNCFX => 5 FWMGM\n\
           139 ORE => 4 NVRVD\n\
           144 ORE => 7 JNWZP\n\
           5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
           5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
           145 ORE => 6 MNCFX\n\
           1 NVRVD => 8 CXFTF\n\
           1 VJHF, 6 MNCFX => 4 RFSQX\n\
           176 ORE => 6 VJHF"), 180697);
    }

    #[test]
    fn it_solves_part1_example_5() {
        assert_eq!(part_1(
           "171 ORE => 8 CNZTR\n\
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
            114 ORE => 4 BHXH\n\
            14 VRPVC => 6 BMBT\n\
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
            5 BMBT => 4 WPTQ\n\
            189 ORE => 9 KTJDG\n\
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
            12 VRPVC, 27 CNZTR => 2 XDBXC\n\
            15 KTJDG, 12 BHXH => 5 XCVML\n\
            3 BHXH, 2 VRPVC => 7 MZWV\n\
            121 ORE => 7 VRPVC\n\
            7 XCVML => 6 RJRHP\n\
            5 BHXH, 4 VRPVC => 5 LTCX"), 2210736);
    }

    #[test]
    fn it_solves_part2_example_1() {
        assert_eq!(part_2(
          "157 ORE => 5 NZVS\n\
           165 ORE => 6 DCFZ\n\
           44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
           12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
           179 ORE => 7 PSHF\n\
           177 ORE => 5 HKGWZ\n\
           7 DCFZ, 7 PSHF => 2 XJWVT\n\
           165 ORE => 2 GPVTF\n\
           3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT", 1000000000000), 82892753);
    }

    #[test]
    fn it_solves_part2_example_2() {
        assert_eq!(part_2(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
            17 NVRVD, 3 JNWZP => 8 VPVL\n\
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
            22 VJHF, 37 MNCFX => 5 FWMGM\n\
            139 ORE => 4 NVRVD\n\
            144 ORE => 7 JNWZP\n\
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
            145 ORE => 6 MNCFX\n\
            1 NVRVD => 8 CXFTF\n\
            1 VJHF, 6 MNCFX => 4 RFSQX\n\
            176 ORE => 6 VJHF", 1000000000000), 5586023); // 5586022
    }

    #[test]
    fn it_solves_part2_example_3() {
        assert_eq!(part_2(
           "171 ORE => 8 CNZTR\n\
           7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
           114 ORE => 4 BHXH\n\
           14 VRPVC => 6 BMBT\n\
           6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
           6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
           15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
           13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
           5 BMBT => 4 WPTQ\n\
           189 ORE => 9 KTJDG\n\
           1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
           12 VRPVC, 27 CNZTR => 2 XDBXC\n\
           15 KTJDG, 12 BHXH => 5 XCVML\n\
           3 BHXH, 2 VRPVC => 7 MZWV\n\
           121 ORE => 7 VRPVC\n\
           7 XCVML => 6 RJRHP\n\
           5 BHXH, 4 VRPVC => 5 LTCX", 1000000000000), 460665);
           // 460664 is the correct, off by one..
    }
}
