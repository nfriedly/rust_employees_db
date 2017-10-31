extern crate regex;

pub mod company;
pub mod parser;

#[derive(Debug,PartialEq)]
pub enum Action {
    AddEmployeeToDepartment(String, String),
    ListEmployeesInDepartment(String),
    ListAllEmployees,
    Unknown
}

