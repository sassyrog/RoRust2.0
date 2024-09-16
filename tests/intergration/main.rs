#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
