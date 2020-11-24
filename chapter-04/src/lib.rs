#[cfg(test)]
use mockall::{automock, predicate::*};

pub struct PriceCalculator {
    price_list: Box<dyn PriceList>,
    delivery_service: Box<dyn DeliveryService>,
}

impl PriceCalculator {
    pub fn new(price_list: Box<dyn PriceList>, delivery_service: Box<dyn DeliveryService>) -> Self {
        Self {
            price_list,
            delivery_service,
        }
    }

    pub fn calculate(&self, purchase: &Purchase) -> f64 {
        let discount = self.price_list.discount_rate(purchase.value);
        let shipping_cost = self.delivery_service.shipping_cost(&purchase.city);
        let value = purchase.value * (1.0 - discount) + shipping_cost;
        f64::trunc(value * 100.0) / 100.0
    }
}

pub struct Purchase {
    pub value: f64,
    pub city: String,
}

#[cfg_attr(test, automock)]
pub trait PriceList {
    fn discount_rate(&self, value: f64) -> f64;
}

#[cfg_attr(test, automock)]
pub trait DeliveryService {
    fn shipping_cost(&self, city: &str) -> f64;
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_price() {
        let mut price_list = MockPriceList::new();
        let mut delivery_service = MockDeliveryService::new();

        price_list
            .expect_discount_rate()
            .with(gt(5000.0))
            .return_const(0.03);
        price_list
            .expect_discount_rate()
            .with(gt(1000.0))
            .return_const(0.05);
        price_list.expect_discount_rate().return_const(0.0);

        delivery_service
            .expect_shipping_cost()
            .with(eq("SP"))
            .return_const(15.0);
        delivery_service.expect_shipping_cost().return_const(30.0);

        let calculator = PriceCalculator::new(Box::new(price_list), Box::new(delivery_service));

        let purchases = vec![
            (50.0, String::from("SP"), 65.0),
            (50.0, String::from("POA"), 80.0),
            (5200.3, String::from("SP"), 5059.29),
            (2333.85, String::from("SP"), 2232.15),
        ];

        purchases.iter().for_each(|(value, city, expected)| {
            let purchase = Purchase {
                value: *value,
                city: city.to_owned(),
            };
            let actual = calculator.calculate(&purchase);
            assert_eq!(*expected, actual);
        });
    }
}
