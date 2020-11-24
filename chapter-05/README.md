# Chapter 05 - The encapsulation and propagation of changes

**BoletosProcessor.java**
```java
import java.util.List;

public class BoletosProcessor {

    public void process(final List<Boleto> boletos, final Invoice invoice) {
        if (boletos != null && invoice != null) {
            boletos.forEach(boleto -> {
                final var payment = new Payment(boleto.getValue(), PaymentMethod.BOLETO);
                invoice.addPayment(payment);
            });
        }
    }

}
```

**PaymentMethod.java**
```java
public enum PaymentMethod {

    BOLETO,
    CreditCard,
    EWALLET,
    MOBILE
    ;

}
```

**Boleto.java**
```java
public class Boleto {

    private double value;

    public Boleto(final double value) {
        this.value = value;
    }

    public double getValue() {
        return this.value;
    }

    public void setValue(final double value) {
        this.value = value;
    }

}
```

**Invoice.java**
```java
import java.util.Collections;
import java.util.Collectors;
import java.util.ArrayList;
import java.util.List;

public class Invoice {

    private double value;

    private List<Payment> payments;

    private boolean paid;

    public Invoice(final double value) {
        this.value = value;
        this.payments = new ArrayList<>();
    }

    public double getValue() {
        return this.value;
    }

    public List<Payment> getPayments() {
        return Collections.unmodifiableList(this.payments);
    }

    public boolean isPaid() {
        return this.paid;
    }

    public void addPayment(final Payment payment) {
        if (payment != null) {
            this.payments.add(payment);

            if (this.totalAmountPaid() >= this.value) {
                this.paid = true;
            }
        }
    }

    private double totalAmountPaid() {
        return this.payments.stream().collect(Collectors.summingDouble(Payment::getValue));
    }

}
```

**Payment.java**
```java
public class Payment {

    private double value;

    private PaymentMethod paymentMethod;

    public Payment(final double value, final PaymentMethod paymentMethod) {
        this.value = value;
        this.paymentMethod = paymentMethod;
    }

    public double getValue() {
        return this.value;
    }

    public PaymentMethod getPaymentMethod() {
        return this.paymentMethod;
    }

    public void setValue(final double value) {
        this.value = value;
    }

    public void setPaymentMethod(final PaymentMethod paymentMethod) {
        this.paymentMethod = paymentMethod;
    }

}
```

## Run tests

```shell script
$ cargo test --package chapter-05
```

## Run examples

```shell script
$ cargo run --example process
```

```text
Invoice Paid!
```
