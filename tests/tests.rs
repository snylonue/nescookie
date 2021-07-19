#[cfg(test)]
mod tests {
    use nescookie::parse;
    const CONTENT: &str = include_str!("cookies.txt");
    #[test]
    fn cookie() {
        let jar = parse(CONTENT).unwrap();
        assert_eq!(
            jar.get("first_visit_datetime_pc").map(|c| c.value()),
            Some("2021-07-19+10%3A48%3A50")
        );
        assert_eq!(
            jar.get("PHPSESSID").map(|c| c.value()),
            Some("j6amv2igf0cec4fdtld5rre5ud7ig3l2")
        );
    }
}
