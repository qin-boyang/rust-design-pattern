
enum PaymentMethod {
    CreditCard(u32),
    PayPal(String, String),
    Bitcoin(String),
}
impl PaymentMethod {
    fn pay(&self){
        match self {
            PaymentMethod::CreditCard(number) => println!("Paying with credit card: {}", number),
            PaymentMethod::PayPal(email, password) => println!("Paying with PayPal: {} and password: {}", email, password),
            PaymentMethod::Bitcoin(address) => println!("Paying with Bitcoin: {}", address),
        }
    }
}

struct PaymentMethodFactory;
impl PaymentMethodFactory {
    fn create(&self, config: PaymentConfig) -> PaymentMethod {
        match config {
            PaymentConfig::CreditCard { number } => PaymentMethod::CreditCard(number),
            PaymentConfig::PayPal { email, pass } => PaymentMethod::PayPal(email, pass),
            PaymentConfig::Bitcoin { address } => PaymentMethod::Bitcoin(address),
        }
    }
}

enum PaymentConfig {
    CreditCard { number: u32 },
    PayPal { email: String, pass: String },
    Bitcoin { address: String },
}

#[cfg( test)]
mod tests {
    use super::*;
    #[test]
    fn test_payment_method_factory() {
        let config = PaymentConfig::CreditCard {
            number: 123456
        };
        let payment = PaymentMethodFactory.create(config);
        payment.pay();
        let config = PaymentConfig::PayPal {
            email: "test@test.com".to_string(),
            pass: "password".to_string(),
        };
        let payment = PaymentMethodFactory.create(config);
        payment.pay();
        let config = PaymentConfig::Bitcoin {
            address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        };
        let payment = PaymentMethodFactory.create(config);
        payment.pay();
    }
}