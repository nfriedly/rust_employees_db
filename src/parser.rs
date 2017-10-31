use regex::Regex;
use ::Action;

pub fn parse_user_input(input: &str) -> Action {
    // a *proper* parser would operate in a single pass and not need any regexes..
    // but this is good enough for now

    let re = Regex::new(r"^(?i)add (\w+) to (\w+)").unwrap();
    let capts = re.captures(input);
    if let Some(details) = capts {
        return Action::AddEmployeeToDepartment(
            String::from(details.get(1).unwrap().as_str()),
            String::from(details.get(2).unwrap().as_str())
        );
    }

    let re = Regex::new(r"^(?i)list employees in (\w+)").unwrap();
    let capts = re.captures(input);
    if let Some(details) = capts {
        return Action::ListEmployeesInDepartment(
            String::from(details.get(1).unwrap().as_str())
        );
    }

    let re = Regex::new(r"^(?i)list (all )?employees").unwrap();
    if re.is_match(input) {
        return Action::ListAllEmployees;
    }

    Action::Unknown
}


#[cfg(test)]
mod test {
    use super::*;
    use ::Action;

    #[test]
    fn add_user_input () {
        let input = "Add Joe to HR";
        let actual = parse_user_input(input);
        let expected = Action::AddEmployeeToDepartment(
            String::from("Joe"),
            String::from("HR"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn list_dept_input () {
        let input = "list employees in HR";
        let actual = parse_user_input(input);
        let expected = Action::ListEmployeesInDepartment(
            String::from("HR"));
        assert_eq!(actual, expected);
    }


    #[test]
    fn list_emps () {
        let input = "list employees";
        let actual = parse_user_input(input);
        let expected = Action::ListAllEmployees;
        assert_eq!(actual, expected);
    }

    #[test]
    fn list_all_emps () {
        let input = "list all employees";
        let actual = parse_user_input(input);
        let expected = Action::ListAllEmployees;
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_empty () {
        let input = "";
        let actual = parse_user_input(input);
        let expected = Action::Unknown;
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_nonsense () {
        let input = "foobar";
        let actual = parse_user_input(input);
        let expected = Action::Unknown;
        assert_eq!(actual, expected);
    }
}
