use chapter_04::{DeliveryService, PriceCalculator, PriceList, Purchase};

struct DifferentiatedPriceList {}
struct ShippingByPost {}

impl PriceList for DifferentiatedPriceList {
    fn discount_rate(&self, value: f64) -> f64 {
        match value {
            v if v > 5000.0 => 0.03,
            v if v > 1000.0 => 0.05,
            _ => 0.0,
        }
    }
}

impl DeliveryService for ShippingByPost {
    fn shipping_cost(&self, city: &String) -> f64 {
        if "SAO PAULO".to_string().eq(&city.to_uppercase()) {
            15.0
        } else {
            30.0
        }
    }
}

fn main() {
    let calculator = PriceCalculator::new(
        Box::new(DifferentiatedPriceList {}),
        Box::new(ShippingByPost {}),
    );

    let purchases = vec![
        (50.0, String::from("Sao Paulo")),
        (50.0, String::from("Porto Alegre")),
        (5200.3, String::from("Sao Paulo")),
        (2333.85, String::from("Sao Paulo")),
    ];

    purchases.iter().for_each(|(value, city)| {
        let purchase = Purchase {
            value: *value,
            city: city.to_owned(),
        };
        let final_price = calculator.calculate(&purchase);

        println!(
            "Purchase in the amount of $ {:.2} with postage to the city of {}. \
            The total purchase is $ {:.2}.",
            value, purchase.city, final_price
        );
    });
}
