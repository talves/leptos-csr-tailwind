pub trait ClassVariant {
    fn to_string(&self) -> String {
        self.as_vec()
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
    fn as_vec(&self) -> Vec<&'static str>;
}
