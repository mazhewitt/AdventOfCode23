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

fn parse_rule(rule_str: &str) -> Result<Rule, String> {
    let rule_matcher = Regex::new(r"^(\w)([<>])(\d+):(\w+)$").expect("Invalid regex");
    if let Some(captures) = rule_matcher.captures(rule_str) {
        Ok(Rule {
            item: captures.get(1).unwrap().as_str().to_string(),
            operator: captures.get(2).unwrap().as_str().chars().next().unwrap(),
            value: captures.get(3).unwrap().as_str().parse::<i32>().map_err(|e| e.to_string())?,
            action: captures.get(4).unwrap().as_str().to_string(),
        })
    } else {
        Err("Failed to parse rule".to_string())
    }
}

fn parse_workflow(workflow_str: &str) -> Result<WorkFlow, String> {
    let (wf_name, mut wf_rules) = workflow_str.split_once("{").ok_or("Invalid workflow format")?;
    let wf_name = wf_name.to_string();
    wf_rules = wf_rules.trim_end_matches("}");
    let mut rules: Vec<_> = wf_rules.split(",").collect();

    let default = rules.pop().ok_or("No default action in workflow")?.to_string();
    let parsed_rules = rules.into_iter()
        .map(|r| parse_rule(r))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(WorkFlow { name: wf_name, rules: parsed_rules, default })
}

fn main() {
    println!("Hello, world!");
}
fn parse_parts(part_str: &str) -> Vec<(char, usize)> {
    let parse_parts_r = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").expect("Invalid regex");
    let captures = parse_parts_r.captures(part_str).expect("Invalid part string");
    vec![
        ('x', captures.get(1).unwrap().as_str().parse::<usize>().unwrap()),
        ('m', captures.get(2).unwrap().as_str().parse::<usize>().unwrap()),
        ('a', captures.get(3).unwrap().as_str().parse::<usize>().unwrap()),
        ('s', captures.get(4).unwrap().as_str().parse::<usize>().unwrap()),
    ]
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
        assert_eq!(actual.unwrap(), expected);
    }



    #[test]
    fn test_process_part_through_workflows() {

    }

    #[test]
    fn test_parse_parts(){
        let part_str = "{x=787,m=2655,a=1222,s=2876}";
        let expected = vec![
            ('x', 787),
            ('m', 2655),
            ('a', 1222),
            ('s', 2876),
        ];
        let actual = parse_parts(part_str);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_load_data(){
        let filename = "test.txt";

        let (workflows, parts) = load_data(filename);
        assert_eq!(workflows.len(), 11);
        assert_eq!(parts.len(), 5);
    }

    fn load_data(filename: &str) -> (Vec<WorkFlow>, Vec<Vec<(char, usize)>>) {
        let file_contents = std::fs::read_to_string(filename).expect("Failed to read file");
        let workflows_block = file_contents.split("\n\n").next().unwrap();
        let parts_block = file_contents.split("\n\n").next().unwrap();
        let workflows = workflows_block.lines().map(|wf| parse_workflow(wf).unwrap()).collect();
        let parts = parts_block.lines().map(|p| parse_parts(p)).collect();
        (workflows, parts)
    }
}