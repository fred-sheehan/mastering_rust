// enums_method.rs

enum PaymentMode {
    Credit,
    Debit,
    Paypal,
}

// dummy payment handlers

fn pay_by_credit(amt: f64) {
    println!("Processing credit payment of {}", amt);
}

fn pay_by_debit(amt: f64) {
    println!("Processing debit payment of {}", amt);
}

fn paypal_redirect(amt: f64) {
    println!("Processing paypal payment of {}", amt);
}

impl PaymentMode {
    fn pay(&self, amt: f64) {
        // an enumeration method
        match self {
            PaymentMode::Credit => pay_by_credit(amt),
            PaymentMode::Debit => pay_by_debit(amt),
            PaymentMode::Paypal => paypal_redirect(amt),
        }
    }
}

fn get_saved_payment_mode() -> PaymentMode {
    PaymentMode::Credit
}

fn main() {
    let payment_mode = get_saved_payment_mode();
    payment_mode.pay(643.34);
}
