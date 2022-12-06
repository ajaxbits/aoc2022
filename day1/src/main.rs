use std::cmp::Reverse;

use jane_eyre::eyre::Context;

fn main() -> jane_eyre::Result<()> {
    jane_eyre::install()?;

    fn gen_inventory() -> jane_eyre::Result<Vec<u32>> {
        let input = include_str!("./day1.txt").to_string();
        let inventory = input
            .trim()
            .split("\n\n")
            .map(|elf_inventory| -> u32 {
                elf_inventory
                    .split("\n")
                    .map(|itm| itm.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .collect::<Vec<u32>>();
        Ok(inventory)
    }

    fn find_top_three_elves(inventory: Vec<u32>) -> Vec<u32> {
        let mut inventory = inventory;
        inventory.sort();
        inventory.into_iter().rev().take(3).collect()
    }

    let inventory = gen_inventory()?;
    let wealthy_elves = find_top_three_elves(inventory);
    println!("The most burdened elf is carrying {} calories.\nThe next two most burdened elves are carrying {} calories and {} calories, respectively", wealthy_elves[0], wealthy_elves[1], wealthy_elves[2]);

    Ok(())
}
