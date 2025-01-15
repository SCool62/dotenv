#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::*;

    #[test]
    fn it_works() {
        let foo = dotenv!("FOO");
        let bazz = dotenv!("BAZZ");
        assert_eq!(foo, "bar");
        assert_eq!(bazz, "world");
        assert_eq!(None::<&str>, dotenv_option!("FUZZ"));
    }
}
