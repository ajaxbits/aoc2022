fn get_input() -> String {
    include_str!("../input/day1").to_string()
}

pub fn find_elf() -> u32 {
    let input = get_input();
    let mut inventory = input
        .trim()
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .split("\n")
                .map(|itm| itm.parse::<u32>().unwrap())
                .sum()
        })
        .collect::<Vec<u32>>();
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
