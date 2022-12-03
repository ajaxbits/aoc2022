mod day_1 {
    pub fn get_input() -> String {
        include_str!("../input/day1").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use md5;

    #[test]
    fn check_input() {
        let input = day_1::get_input();
        let hash = md5::compute(input);
        let result = format!("{:x}", hash);
        assert_eq!(result, "03b834b41470c2cfd02112fd50f39e30");
    }
}
