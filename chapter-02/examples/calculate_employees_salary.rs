use chapter_02::{Employee, Role, SalaryCalculator};

fn calculate_and_print(employee: &Employee, salary_calculator: &SalaryCalculator) {
    let net_salary = salary_calculator.calculate(employee);
    println!(
        "{}, base salary ${:.2} -> ${:.2}",
        employee.role, employee.base_salary, net_salary
    );
}

fn main() {
    let employees = vec![
        Employee::new(Role::DEVELOPER, 1500.0),
        Employee::new(Role::DEVELOPER, 8500.0),
        Employee::new(Role::DBA, 3000.0),
        Employee::new(Role::DBA, 8000.0),
        Employee::new(Role::TESTER, 2800.0),
        Employee::new(Role::TESTER, 3200.0),
    ];

    let salary_calculator = SalaryCalculator {};

    employees
        .iter()
        .for_each(move |employee| calculate_and_print(&employee, &salary_calculator));
}
