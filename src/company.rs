use std::collections::HashMap;

// todo: try to use &str instead of String more often

#[derive(Debug)]
pub struct Company {
    employees_by_dept: HashMap<String, Vec<String>>
}

impl Company {

    pub fn new() -> Company {
        Company {
            employees_by_dept: HashMap::new()
        }
    }

    pub fn add(&mut self, name: String, dept: String)  {
        let emps = self.employees_by_dept
            .entry(dept)
            .or_insert(Vec::new());
        emps.push(name);
        emps.sort();
    }

    pub fn employees_in(&self, dept: &str) -> Vec<String> {
        let emps = self.employees_by_dept.get(dept);
        match emps {
            Some(emps) => emps.clone(),
            None => Vec::new()
        }
    }

    pub fn departments(&self) -> Vec<&str> {
        let mut deps: Vec<&str> = Vec::new();
        for dep in self.employees_by_dept.keys() {
            deps.push(dep);
        }
        deps.sort();
        deps
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_name() {
        let mut comp = Company::new();
        comp.add(String::from("Joe"), String::from("HR"));
        let actual = comp.employees_by_dept.get(&String::from("HR")).unwrap();
        let expected = &vec![String::from("Joe")];
        assert_eq!(actual, expected);
    }

    #[test]
    fn names_sorted_within_dept() {
        let mut comp = Company::new();
        comp.add(String::from("B"), String::from("HR"));
        comp.add(String::from("Z"), String::from("HR"));
        comp.add(String::from("A"), String::from("HR"));
        comp.add(String::from("K"), String::from("HR"));
        let actual = comp.employees_by_dept.get(&String::from("HR")).unwrap();
        let expected = &vec![
            String::from("A"),
            String::from("B"),
            String::from("K"),
            String::from("Z"),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn depts_sorted() {
        let mut comp = Company::new();
        comp.add(String::from("Joe"), String::from("Procurement"));
        comp.add(String::from("Joe"), String::from("Engineering"));
        comp.add(String::from("Joe"), String::from("Sales"));
        comp.add(String::from("Joe"), String::from("HR"));
        let actual = comp.departments();
        let expected = vec![
            "Engineering",
            "HR",
            "Procurement",
            "Sales",
        ];
        assert_eq!(actual, expected);
    }
}
