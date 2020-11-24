use chapter_05::{Boleto, BoletosProcessor, Invoice};

fn main() {
    let processor = BoletosProcessor {};
    let boletos = vec![
        Boleto { value: 1.0 },
        Boleto { value: 2.65 },
        Boleto { value: 4.5 },
        Boleto { value: 7.99 },
    ];
    let mut invoice = Invoice::new(16.14);

    processor.process(&boletos, &mut invoice);

    if invoice.is_paid() {
        println!("Invoice Paid!");
    }
}
