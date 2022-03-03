#[cfg(test)]
mod tests {
    #[test]
    fn check_parse() {
        assert_eq!(crate::parse(&String::from("Test? lub a"), &["lub"], "?"), String::from("Test"));
    }
}