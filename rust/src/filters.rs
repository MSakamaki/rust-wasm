
pub fn has_five_length(name: &str) -> bool {
    name.len() > 5
}

#[cfg(test)]
mod tests {
    use super::has_five_length;

    #[test]
    pub fn it_has_five_length_small() {
        assert!( ! has_five_length("12345") );
    }
    #[test]
    pub fn it_has_five_length_large() {
        assert!( has_five_length("123456") );
    }
}
