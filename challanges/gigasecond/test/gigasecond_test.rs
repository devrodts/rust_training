#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn birthday() {
        let start = datetime!(2015-01-24 22:00:00);
        let expected = datetime!(2046-10-02 23:46:40);
        assert_eq!(after(start), expected);
    }
}
