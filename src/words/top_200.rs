pub fn words() -> Vec<String> {
    include_str!("top-200.csv")
        .lines()
        .map(str::to_owned)
        .collect::<Vec<String>>()
}
