pub mod prelude {
    pub struct ParseLines<'a, T> {
        inner: std::str::Lines<'a>,
        _data: std::marker::PhantomData<T>,
    }

    impl<'a, T> ParseLines<'a, T> {
        fn new(input: &'a str) -> Self {
            Self {
                inner: input.lines(),
                _data: std::marker::PhantomData,
            }
        }
    }

    impl<T> Iterator for ParseLines<'_, T>
    where
        T: std::str::FromStr<Err = std::convert::Infallible>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(|line| T::from_str(line).unwrap())
        }
    }

    pub fn parse_lines<T>(input: &str) -> ParseLines<T> {
        ParseLines::new(input)
    }
}
