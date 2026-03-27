struct AritmeticnoZaporedje <T> {
    ime: String,
    a0: T,
    d: T,
    index: u64,
}

impl AritmeticnoZaporedje <T> {
    fn new(ime: &str, a0: T, d: T) -> Self {
        AritmeticnoZaporedje {
            ime: ime.to_string(),
            a0,
            d,
            index: 0
        }
    }
}

impl<T> Zaporedje<T> for AritmeticnoZaporedje {

}