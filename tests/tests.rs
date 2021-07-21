#[cfg(test)]
mod tests {
    use nescookie::parse;
    const CONTENT: &str = include_str!("cookies.txt");
    #[test]
    fn cookie() {
        let jar = parse(CONTENT).unwrap();
        assert_eq!(jar.iter().count(), 6);
        assert_eq!(
            jar.get("first_visit_datetime_pc").map(|c| c.value()),
            Some("2021-07-19+10%3A48%3A50")
        );
        assert!(jar.get("p_ab_id").map(|c| c.secure()).flatten().unwrap());
        assert_eq!(
            jar.get("PHPSESSID")
                .map(|c| c.expires_datetime().unwrap().unix_timestamp()),
            Some(1626662932)
        );
        assert_eq!(jar.get("yuid_b").map(|c| c.path()).flatten(), Some("/"))
    }
}
