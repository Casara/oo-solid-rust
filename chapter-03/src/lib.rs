pub type ActionsAfterGeneratingTaxInvoice<'a> = Vec<Box<&'a dyn ActionAfterGeneratingTaxInvoice>>;

pub struct TaxInvoiceGenerator<'a> {
    pub actions: ActionsAfterGeneratingTaxInvoice<'a>,
}

impl<'a> TaxInvoiceGenerator<'a> {
    pub fn new(actions: ActionsAfterGeneratingTaxInvoice<'a>) -> Self {
        Self { actions }
    }

    pub fn generate(&self, invoice: &Invoice) -> TaxInvoice {
        let value = invoice.monthly_value;
        let tax_invoice = TaxInvoice {
            value,
            simple_tax: self.simple_tax_on_the(value),
        };

        self.actions
            .iter()
            .for_each(|action| action.execute(&tax_invoice));

        tax_invoice
    }

    fn simple_tax_on_the(&self, value: f64) -> f64 {
        f64::trunc(value * 0.06 * 100.0) / 100.0
    }
}

pub struct Invoice {
    pub monthly_value: f64,
}

pub struct TaxInvoice {
    pub value: f64,
    pub simple_tax: f64,
}

pub trait ActionAfterGeneratingTaxInvoice {
    fn execute(&self, tax_invoice: &TaxInvoice);
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;
    use std::cell::Cell;

    struct EmailSender {
        pub executed: Cell<bool>,
    }

    struct TaxInvoiceDao {
        pub executed: Cell<bool>,
    }

    impl ActionAfterGeneratingTaxInvoice for EmailSender {
        fn execute(&self, _tax_invoice: &TaxInvoice) {
            self.executed.set(true);
        }
    }

    impl ActionAfterGeneratingTaxInvoice for TaxInvoiceDao {
        fn execute(&self, _tax_invoice: &TaxInvoice) {
            self.executed.set(true);
        }
    }

    #[test]
    fn test_generate_invoice_and_execute_actions() {
        let invoice = Invoice {
            monthly_value: 57.3,
        };

        let action_1 = EmailSender {
            executed: Cell::new(false),
        };
        let action_2 = TaxInvoiceDao {
            executed: Cell::new(false),
        };

        let tax_invoice_generator =
            TaxInvoiceGenerator::new(vec![Box::new(&action_1), Box::new(&action_2)]);

        let tax_invoice = tax_invoice_generator.generate(&invoice);

        assert_eq!(57.30, tax_invoice.value);
        assert_eq!(3.43, tax_invoice.simple_tax);
        assert_eq!(true, action_1.executed.get());
        assert_eq!(true, action_2.executed.get());
    }
}
