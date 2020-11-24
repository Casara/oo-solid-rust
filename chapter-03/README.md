# Chapter 03 - Coupling and the so-called DIP (Dependency Inversion Principle)

**Invoice.java**
```java
public class Invoice {

    private double monthlyValue;

    public Invoice(final double monthlyValue) {
        this.monthlyValue = monthlyValue;
    }

    public double getMonthlyValue() {
        return this.monthlyValue;
    }

    public void setMonthlyValue(final double monthlyValue) {
        this.monthlyValue = monthlyValue;
    }

}
```

**TaxInvoice.java**
```java
public class TaxInvoice {

    private double value;

    private double simpleTax;

    public TaxInvoice(final double value, final double simpleTax) {
        this.value = value;
        this.simpleTax = simpleTax;
    }

    public double getValue() {
        return this.value;
    }

    public double getSimpleTax() {
        return this.simpleTax;
    }

    public void setValue(final double value) {
        this.value = value;
    }

    public void setSimpleTax(final double simpleTax) {
        this.simpleTax = simpleTax;
    }

}
```

**TaxInvoiceGenerator.java**
```java
public class TaxInvoiceGenerator {

    private final List<ActionAfterGeneratingTaxInvoice> actions;

    public TaxInvoiceGenerator(final List<ActionAfterGeneratingTaxInvoice> actions) {
        this.actions = actions;
    }

    public TaxInvoice generate(final Invoice invoice) {
        final var value = invoice.getMonthlyValue();
        final var taxInvoice = new TaxInvoice(value, this.simpleTaxOnThe(value));

        this.actions.forEach(action -> action.execute(taxInvoice));

        return taxInvoice;
    }

    private double simpleTaxOnThe(final double value) {
        return value * 0.06;
    }

}
```

**ActionAfterGeneratingTaxInvoice.java**
```java
@FunctionalInterface
public interface ActionAfterGeneratingTaxInvoice {

    void execute(TaxInvoice taxInvoice);

    default ActionAfterGeneratingTaxInvoice andThen(final ActionAfterGeneratingTaxInvoice after) {
        Objects.requireNonNull(after);
        return (TaxInvoice taxInvoice) -> {
            this.execute(taxInvoice);
            after.execute(taxInvoice);
        };
    }

}
```

**EmailSender.java**
```java
public class EmailSender implements ActionAfterGeneratingTaxInvoice {

    @Override
    public void execute(final TaxInvoice taxInvoice) {
        // implementation
    }

}
```

**Sap.java**
```java
public class Sap implements ActionAfterGeneratingTaxInvoice {

    @Override
    public void execute(final TaxInvoice taxInvoice) {
        // implementation
    }

}
```

**TaxInvoiceDao.java**
```java
public class TaxInvoiceDao implements ActionAfterGeneratingTaxInvoice {

    @Override
    public void execute(final TaxInvoice taxInvoice) {
        // implementation
    }

}
```

## Run tests

```shell script
$ cargo test --package chapter-03
```

## Run examples

```shell script
$ cargo run --example generate_tax_invoice
```

```text
Tax invoice generated! Value: $ 57.30, Simple tax: $ 3.43.
```
