# Chapter 02 - Cohesion and the so-called SRP (Single-responsiblity principle)

**Role.java**
```java
public enum Role {

    DEVELOPER(new TenOrTwentyPercent()),
    DBA(new FifteenOrTwentyFivePercent()),
    TESTER(new FifteenOrTwentyFivePercent())
    ;

    private CalculationRule rule;

    Role(final CalculationRule rule) {
        this.rule = rule;
    }

    public CalculationRule getRule() {
        return this.rule;
    }

}
```

**Employee.java**
```java
public class Employee {

    private Role role;

    private double baseSalary;

    public Employee(final Role role, final double baseSalary) {
        this.role = role;
        this.baseSalary = baseSalary;
    }

    public Role getRole() {
        return this.role;
    }

    public double getBaseSalary() {
        return this.baseSalary;
    }

    public void setRole(final Role role) {
        this.role = role;
    }

    public void setBaseSalary(final double baseSalary) {
        this.baseSalary = baseSalary;
    }

}
```

**CalculationRule.java**
```java
public interface CalculationRule {

    double calculate(Employee employee);

}
```

**TenOrTwentyPercent.java**
```java
import java.util.Optional;

public class TenOrTwentyPercent implements CalculationRule {

    @Override
    public double calculate(final Employee employee) {
        return Optional.ofNullable(employee)
                .map(Employee::getBaseSalary)
                .map(baseSalary -> {
                    final var discount = baseSalary > 3000.0 ? 0.8 : 0.9;
                    return baseSalary * discount;
                })
                .orElseThrow(() -> new RuntimeException("Invalid employee."));
    }

}
```

**FifteenOrTwentyFivePercent.java**
```java
import java.util.Optional;

public class FifteenOrTwentyFivePercent implements CalculationRule {

    @Override
    public double calculate(final Employee employee) {
        return Optional.ofNullable(employee)
                .map(Employee::getBaseSalary)
                .map(baseSalary -> {
                    final var discount = baseSalary > 2000.0 ? 0.75 : 0.85;
                    return baseSalary * discount;
                })
                .orElseThrow(() -> new RuntimeException("Invalid employee."));
    }

}
```

**SalaryCalculator.java**
```java
import java.util.Objects;
import java.util.Optional;

public class SalaryCalculator {

    public double calculate(final Employee employee) {
        return Optional.ofNullable(employee)
                .map(Employee::getRole)
                .filter(Objects::nonNull)
                .map(Role::getRule)
                .map(rule -> rule.calculate(employee))
                .orElseThrow(() -> new RuntimeException("Invalid employee."));
    }

}
```

## Run tests

```shell script
$ cargo test --package chapter-02
```

## Run examples

```shell script
$ cargo run --example calculate_employees_salary
```

```text
DEVELOPER, base salary $1500.00 -> $1350.00
DEVELOPER, base salary $8500.00 -> $6800.00
DBA, base salary $3000.00 -> $2250.00
DBA, base salary $8000.00 -> $6000.00
TESTER, base salary $2800.00 -> $2100.00
TESTER, base salary $3200.00 -> $2400.00
```
