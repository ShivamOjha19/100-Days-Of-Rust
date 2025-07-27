use std::collections::HashMap;
pub struct CurrencyConverter {
    rates: HashMap<String, f64>,
}
impl CurrencyConverter {
    pub fn new() -> Self {
        let mut rates = HashMap::new();
        rates.insert("USD".to_string(), 1.0);
        rates.insert("EUR".to_string(), 0.85);
        rates.insert("INR".to_string(), 74.5);
        rates.insert("JPY".to_string(), 110.0);
        CurrencyConverter { rates }
    }
   
    pub fn convert(&self, from: &str, to: &str, amount: f64) -> f64 {
        let from_rate = self.rates.get(from)
            .unwrap_or_else(|| panic!("Unsupported currency: {}", from));
        let to_rate = self.rates.get(to)
            .unwrap_or_else(|| panic!("Unsupported currency: {}", to));
        let amount_in_usd = amount / from_rate;
        let converted = amount_in_usd * to_rate;
        (converted * 100.0).round() / 100.0 
    }
}
fn main() {
    let converter = CurrencyConverter::new();
    let converted = converter.convert("EUR", "INR", 100.0);
    println!("€100 is ₹{converted} at current rates.");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_usd_to_inr() {
        let converter = CurrencyConverter::new();
        assert_eq!(converter.convert("USD", "INR", 1.0), 74.5);
    }
    #[test]
    fn test_eur_to_inr() {
        let converter = CurrencyConverter::new();
        assert_eq!(converter.convert("EUR", "INR", 1.0), 87.65);
    }
    #[test]
    fn test_jpy_to_usd() {
        let converter = CurrencyConverter::new();
        assert_eq!(converter.convert("JPY", "USD", 110.0), 1.0);
    }
    #[test]
    #[should_panic(expected = "Unsupported currency: BTC")]
    fn test_unsupported_currency() {
        let converter = CurrencyConverter::new();
        converter.convert("BTC", "USD", 1.0); 
    }
}