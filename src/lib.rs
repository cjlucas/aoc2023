pub mod prelude {
    pub fn parse_lines<T: std::str::FromStr<Err = std::convert::Infallible>>(
        input: &str,
    ) -> Vec<T> {
        input
            .lines()
            .map(|line| T::from_str(line).unwrap())
            .collect()
    }
}
