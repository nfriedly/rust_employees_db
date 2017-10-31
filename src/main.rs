extern crate employees_db;

use employees_db::Action::*;
use employees_db::company::Company;
use employees_db::parser::parse_user_input;
use std::io;

fn main() {
  let mut comp = Company::new();

  loop {
    println!("What would you like to do?");

    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let action = parse_user_input(input.as_str());

    match action {
      AddEmployeeToDepartment(name, department) => comp.add(name, department),
      ListEmployeesInDepartment(department) => print_employees_in_department(&comp, department.as_str()),
      ListAllEmployees => print_all_employees(&comp),
      Unknown => break
    }
  }
}


fn print_employees_in_department(company: &Company, department: &str) {
  println!("{}", department);
  for employee in company.employees_in(department) {
    println!(" - {}", employee);
  }
}

fn print_all_employees(company: &Company) {
  for department in company.departments() {
    print_employees_in_department(company, department);
  }
}
