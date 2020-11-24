#[derive(Clone)]
pub enum PaymentMethod {
    Boleto,
    CreditCard,
    Ewallet,
    Mobile,
}

pub struct Boleto {
    pub value: f64,
}

#[derive(Clone)]
pub struct Payment {
    pub value: f64,
    pub payment_method: PaymentMethod,
}

pub struct Invoice {
    value: f64,
    payments: Vec<Payment>,
    paid: bool,
}

impl Invoice {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            payments: vec![],
            paid: false,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn payments(&self) -> Vec<Payment> {
        self.payments.clone()
    }

    pub fn is_paid(&self) -> bool {
        self.paid
    }

    pub fn add_payment(&mut self, payment: Payment) {
        self.payments.push(payment);

        if self.total_amount_paid() >= self.value {
            self.paid = true
        }
    }

    fn total_amount_paid(&self) -> f64 {
        self.payments.iter().map(|payment| payment.value).sum()
    }
}

pub struct BoletosProcessor;

impl BoletosProcessor {
    pub fn process(&self, boletos: &[Boleto], invoice: &mut Invoice) {
        boletos.iter().for_each(|boleto| {
            let payment = Payment {
                value: boleto.value,
                payment_method: PaymentMethod::Boleto,
            };
            invoice.add_payment(payment);
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() {
        let processor = BoletosProcessor {};
        let mut invoice = Invoice::new(16.14);

        assert_eq!(16.14, invoice.value());
        assert_eq!(true, invoice.payments().is_empty());
        assert_eq!(false, invoice.is_paid());

        processor.process(
            &vec![Boleto { value: 1.0 }, Boleto { value: 2.65 }],
            &mut invoice,
        );
        assert_eq!(2, invoice.payments().len());
        assert_eq!(false, invoice.is_paid());

        processor.process(
            &vec![Boleto { value: 4.5 }, Boleto { value: 7.99 }],
            &mut invoice,
        );
        assert_eq!(4, invoice.payments().len());
        assert_eq!(true, invoice.is_paid());
    }
}
