use regex::Regex;


#[derive(Debug, PartialEq)]
struct WorkFlow {
    name: String,
    rules: Vec<Rule>,
    default: String
}
#[derive(Debug, PartialEq)]
struct Rule {
    item: String,
    operator: char,
    value: i32,
    action: String
}

fn parse_workflow(workflow_str: &str) -> WorkFlow {
    //ex{x>10:one,m<20:two,a>30:R,A}
    let mut workflow = WorkFlow {
        name: "".to_string(),
        rules: vec![],
        default: "".to_string(),
    };
    let (wf_name, mut wf_rules) = workflow_str.split_once("{").unwrap();
    workflow.name = wf_name.to_string();
    wf_rules = wf_rules.trim_end_matches("}");
    let mut rules: Vec<_> = wf_rules.split(",").collect();

    // deal with the default rule
    let default = rules.pop().unwrap();
    workflow.default = default.to_string();

    // deal with the rest of the rules
    for mut rule_str in rules {
        let rule_matcher = Regex::new(r"^(\w)([<>])(\d+):(\w+)$").expect("Invalid regex");
        let captures = rule_matcher.captures(rule_str).unwrap();

        let item_str = captures.get(1).unwrap().as_str();
        let operator_str = captures.get(2).unwrap().as_str();
        let value_str = captures.get(3).unwrap().as_str();
        let action_str = captures.get(4).unwrap().as_str();


        let rule = Rule {
            item: item_str.to_string(),
            operator: operator_str.chars().next().unwrap(),
            value: value_str.parse::<i32>().unwrap(),
            action: action_str.to_string()
        };
        workflow.rules.push(rule);
    }
    workflow
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_workflow() {
        let input = "ex{x>10:one,m<20:two,a>30:R,A}";
        let expected = WorkFlow {
            name: "ex".to_string(),
            rules: vec![
                Rule {
                    item: "x".to_string(),
                    operator: '>',
                    value: 10,
                    action: "one".to_string(),
                },
                Rule {
                    item: "m".to_string(),
                    operator: '<',
                    value: 20,
                    action: "two".to_string(),
                },
                Rule {
                    item: "a".to_string(),
                    operator: '>',
                    value: 30,
                    action: "R".to_string(),
                }
            ],
            default: "A".to_string(),
        };

        let actual = parse_workflow(input);
        assert_eq!(actual, expected);
    }


}