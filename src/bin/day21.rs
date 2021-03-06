use std::collections::HashMap;
use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../../inputs/day21.txt");

fn parse_line(input: &str) -> (HashSet<&str>, HashSet<&str>) {
    let mut iter = input.split("(contains ");

    let ingredients: HashSet<&str> = iter.next().unwrap().split_whitespace().collect();

    let allergens: HashSet<&str> = iter
        .next()
        .unwrap()
        .split(')')
        .next()
        .unwrap()
        .split(", ")
        .collect();

    (ingredients, allergens)
}

fn part_1(input: &str) -> usize {
    let mut mappings: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut all_ingredients: Vec<HashSet<&str>> = Vec::new();

    for line in input.lines() {
        let (ingredients, allergens) = parse_line(line);

        all_ingredients.push(ingredients.clone());

        for allergen in allergens.iter() {
            let current_mapping = mappings
                .entry(allergen.clone())
                .or_insert(ingredients.clone());

            *current_mapping = current_mapping
                .intersection(&ingredients)
                .cloned()
                .collect();
        }
    }

    let all_possible_ingredients: HashSet<&str> = mappings
        .iter()
        .flat_map(|(_a, ingredients)| ingredients)
        .cloned()
        .collect();

    all_ingredients
        .into_iter()
        .map(|line| {
            line.into_iter()
                .filter(|ingredient| !all_possible_ingredients.contains(ingredient))
                .count()
        })
        .sum()
}

fn part_2(input: &str) -> String {
    let mut mappings: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut all_ingredients: Vec<HashSet<&str>> = Vec::new();

    for line in input.lines() {
        let (ingredients, allergens) = parse_line(line);

        all_ingredients.push(ingredients.clone());

        for allergen in allergens.iter() {
            let current_mapping = mappings
                .entry(allergen.clone())
                .or_insert(ingredients.clone());

            *current_mapping = current_mapping
                .intersection(&ingredients)
                .cloned()
                .collect();
        }
    }

    let mut final_mappings: Vec<(&str, &str)> = Vec::new();

    loop {
        // Lets hope we can always find one
        let current_mappings = mappings.clone();
        let best_guess = current_mappings
            .iter()
            .find(|(_a, ingredients)| ingredients.len() == 1);

        match best_guess {
            Some((allergen, ingredients)) => {
                mappings.remove(allergen);
                mappings = mappings
                    .into_iter()
                    .map(|(a, i)| (a, i.difference(ingredients).cloned().collect()))
                    .collect();

                final_mappings.push((allergen, ingredients.iter().next().unwrap()));
            }
            None => break, // we're done?
        }
    }

    final_mappings.sort_by_key(|(allergen, _i)| *allergen);
    final_mappings.into_iter().map(|(_a, ingredient)| ingredient).collect::<Vec<&str>>().join(",")
}

fn main() -> () {
    let part_1_result = part_1(INPUT_FILE);
    let part_2_result = part_2(INPUT_FILE);

    println!("[INFO]: Part 1: {:?}", part_1_result);
    println!("[INFO]: Part 2: {:?}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            part_1(
                "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
                 trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
                 sqjhc fvjkl (contains soy)\n\
                 sqjhc mxmxvkd sbzzf (contains fish)"
            ),
            5
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            part_2(
                "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
                 trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
                 sqjhc fvjkl (contains soy)\n\
                 sqjhc mxmxvkd sbzzf (contains fish)"
            ),
            "mxmxvkd,sqjhc,fvjkl"
        )
    }
}
