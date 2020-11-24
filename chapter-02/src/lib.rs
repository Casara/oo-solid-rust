use derive_more::Display;

pub struct SalaryCalculator {}

impl SalaryCalculator {
    pub fn calculate(&self, employee: &Employee) -> f64 {
        Box::<dyn CalculationRule>::from(employee.role.clone()).calculate(employee)
    }
}

pub struct Employee {
    pub role: Role,
    pub base_salary: f64,
}

impl Employee {
    pub fn new(role: Role, base_salary: f64) -> Self {
        Self { role, base_salary }
    }
}

#[derive(Clone, Display)]
pub enum Role {
    DEVELOPER,
    DBA,
    TESTER,
}

impl From<Role> for Box<dyn CalculationRule> {
    fn from(role: Role) -> Box<dyn CalculationRule> {
        match role {
            Role::DEVELOPER => Box::new(TenOrTwentyPercent {}),
            Role::DBA | Role::TESTER => Box::new(FifteenOrTwentyFivePercent {}),
        }
    }
}

struct TenOrTwentyPercent {}
struct FifteenOrTwentyFivePercent {}

trait CalculationRule {
    fn calculate(&self, employee: &Employee) -> f64;
}

impl CalculationRule for TenOrTwentyPercent {
    fn calculate(&self, employee: &Employee) -> f64 {
        if employee.base_salary > 3000.0 {
            &employee.base_salary * 0.8
        } else {
            &employee.base_salary * 0.9
        }
    }
}

impl CalculationRule for FifteenOrTwentyFivePercent {
    fn calculate(&self, employee: &Employee) -> f64 {
        if employee.base_salary > 2000.0 {
            &employee.base_salary * 0.75
        } else {
            &employee.base_salary * 0.85
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_calculate_for_employees() {
        let employees = vec![
            (Employee::new(Role::DEVELOPER, 1500.0), 1350.0),
            (Employee::new(Role::DEVELOPER, 8500.0), 6800.0),
            (Employee::new(Role::DBA, 3000.0), 2250.0),
            (Employee::new(Role::DBA, 8000.0), 6000.0),
            (Employee::new(Role::TESTER, 2800.0), 2100.0),
            (Employee::new(Role::TESTER, 3200.0), 2400.0),
        ];

        let salary_calculator = SalaryCalculator {};

        employees
            .iter()
            .for_each(|(employee, expected_net_salary)| {
                let calculated_net_salary = salary_calculator.calculate(employee);
                assert_eq!(expected_net_salary, &calculated_net_salary);
            });
    }
}
