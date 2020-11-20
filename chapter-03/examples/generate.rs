use chapter_03::{ActionAfterGeneratingTaxInvoice, Invoice, TaxInvoice, TaxInvoiceGenerator};

struct PrintTaxInvoice {}

impl ActionAfterGeneratingTaxInvoice for PrintTaxInvoice {
    fn execute(&self, tax_invoice: &TaxInvoice) {
        println!(
            "Tax invoice generated! Value: $ {:.2}, Simple tax: $ {:.2}.",
            tax_invoice.value, tax_invoice.simple_tax
        );
    }
}

fn main() {
    let invoice = Invoice {
        monthly_value: 57.3,
    };

    let tax_invoice_generator = TaxInvoiceGenerator::new(vec![Box::new(&PrintTaxInvoice {})]);

    tax_invoice_generator.generate(&invoice);
}
