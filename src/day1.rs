use std::collections::HashMap;

fn get_input() -> String {
    include_str!("../input/day1").to_string()
}

pub fn gen_inventory() -> HashMap<u32, Vec<u32>> {
    let input = get_input();
    let inventory = input
        .trim()
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .split("\n")
                .map(|itm| itm.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut manifest = HashMap::new();
    let mut elf_id: u32 = 0;

    for elf_inv in inventory {
        manifest.insert(elf_id, elf_inv);
    }

    manifest
}

pub fn find_elf(manifest: HashMap<u32, Vec<u32>>) -> u32 {
    let inventory: Vec<u32> = manifest.into_values().map(|lst| lst.iter().sum()).collect();
    inventory.sort_unstable();
    inventory.reverse();
    inventory[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use md5;

    #[test]
    fn check_input() {
        let input = get_input();
        let hash = md5::compute(input);
        let result = format!("{:x}", hash);
        assert_eq!(result, "03b834b41470c2cfd02112fd50f39e30");
    }
}
