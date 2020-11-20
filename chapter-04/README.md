# Chapter 04 — Open classes and the so-called OCP (Open-closed principle)

**PriceCalculator.java**
```java
public class PriceCalculator {

    private final PriceList list;

    private final DeliveryService delivery;

    public PriceCalculator(final PriceList list, final DeliveryService delivery) {
        this.list = list;
        this.delivery = delivery;
    }

    public double calculate(final Purchase purchase) {
        return Optional.ofNullable(purchase).map(p -> {
            final var discount = this.list.discountRate(p.getValue());
            final var shippingCost = this.delivery.shippingCost(p.getCity());
            return p.getValue() * (1 - discount) + shippingCost;
        }).orElse(0.0);
    }

}
```

**PriceList.java**
```java
public interface PriceList {

    double discountRate(double value);

}
```

**DeliveryService.java**
```java
public interface DeliveryService {

    double shippingCost(String city);

}
```

**DifferentiatedPriceList.java**
```java
public class DifferentiatedPriceList implements PriceList {

    @Override
    public double discountRate(final double value) {
        if (value > 5000) return 0.03;
        if (value > 1000) return 0.05;
        return 0;
    }

}
```

**ShippingByPost.java**
```java
public class ShippingByPost implements DeliveryService {

    @Override
    public double shippingCost(final String city) {
        if ("SAO PAULO".equals(city.toUpperCase())) {
            return 15;
        }
        return 30;
    }

}
```

**Purchase.java**
```java
public class Purchase {

    private double value;

    private String city;

    public Purchase(final double value, final String city) {
        this.value = value;
        this.city = city;
    }

    public double getValue() {
        return this.value;
    }

    public String getCity() {
        return this.city;
    }

    public void setValue(final double value) {
        this.value = value;
    }

    public void setCity(final String city) {
        this.city = city;
    }

}
```

**PriceCalculatorTests.java**
```java
@ExtendWith(MockitoExtension.class)
public class PriceCalculatorTests {

    @Mock
    private PriceList list;

    @Mock
    private DeliveryService delivery;

    @InjectMocks
    private PriceCalculator calculator;

    @Test
    void test_calculate_must_return_value_with_discount_and_shipping() {
        final var purchase = new Purchase(50.0, "SP");

        when(this.list.discountRate(50.0)).thenReturn(0.1);
        when(this.delivery.shippingCost("SP")).thenReturn(10.0);

        final var value = this.calculator.calculate(purchase);

        assertEquals(55.0, value, 0.0001);
    }

}
```

## Run tests

```shell script
$ cargo test --package chapter-04
```

## Run examples

```shell script
$ cargo run --example calculate_price
```

```text
Purchase in the amount of $ 50.00 with postage to the city of Sao Paulo. The total purchase is $ 65.00.
Purchase in the amount of $ 50.00 with postage to the city of Porto Alegre. The total purchase is $ 80.00.
Purchase in the amount of $ 5200.30 with postage to the city of Sao Paulo. The total purchase is $ 5059.29.
Purchase in the amount of $ 2333.85 with postage to the city of Sao Paulo. The total purchase is $ 2232.15.
```
