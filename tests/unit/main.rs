#[cfg(all(test, feature = "unit"))]
mod unit_tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}