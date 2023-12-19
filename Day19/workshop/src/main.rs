use std::collections::HashMap;
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
    value: usize,
    action: String
}

fn parse_rule(rule_str: &str) -> Result<Rule, String> {
    let rule_matcher = Regex::new(r"^(\w)([<>])(\d+):(\w+)$").expect("Invalid regex");
    if let Some(captures) = rule_matcher.captures(rule_str) {
        Ok(Rule {
            item: captures.get(1).unwrap().as_str().to_string(),
            operator: captures.get(2).unwrap().as_str().chars().next().unwrap(),
            value: captures.get(3).unwrap().as_str().parse::<usize>().map_err(|e| e.to_string())?,
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

fn load_data(filename: &str) -> (HashMap<String, WorkFlow>, Vec<Vec<(char, usize)>>) {
    let file_contents = std::fs::read_to_string(filename).expect("Failed to read file");
    let splitter = Regex::new(r"\r?\n\r?\n").expect("Invalid regex");
    let mut blocks = splitter.split(&file_contents);
    let workflows_block = blocks.next().expect("No workflows block");
    let parts_block = blocks.next().expect("No parts block");
    let workflows = workflows_block.lines().map(|wf| parse_workflow(wf).unwrap()).map(|wf| (wf.name.clone(), wf)).collect();
    let parts = parts_block.lines().map(|p| parse_parts(p)).collect();
    (workflows, parts)
}

fn main() {
    let filename = "input.txt";
    let (workflows, parts) = load_data(filename);
    let results = process_parts_through_workflows(&workflows, &parts);
    let sum_of_accepted = results.iter().filter(|(item, _)| item == &'A').map(|(_, value)| value).sum::<usize>();
    println!("Results: {:}", sum_of_accepted);
    let mut ranges = HashMap::from([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]);

    let total_combinations = calculate_accepted_combinations("in", &workflows, ranges);
    println!("Total combinations: {}", total_combinations);

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

fn process_parts_through_workflows(workflows: &HashMap<String, WorkFlow>, parts: &Vec<Vec<(char, usize)>>) -> Vec<(char, usize)> {
    parts.iter().map(|part| {
        process_part_through_workflows("in", workflows, part)
    }).collect()
}

fn process_part_through_workflows(wf_new:&str, workflows: &HashMap<String, WorkFlow>, part: &Vec<(char, usize)>) -> (char, usize) {
    let workflow = workflows.get(wf_new).expect("Workflow not found");
    let res = process_workflow(workflow, part);
    let part_total = part.iter().map(|(_, value)| value).sum::<usize>();
    match (res.as_str()) {
        "A" => ('A', part_total),
        "R" => ('R', part_total),
        _ => process_part_through_workflows(&res, workflows, part)
    }
}

fn process_workflow(workflow: &WorkFlow, part: &Vec<(char, usize)>) -> String {
    for rule in &workflow.rules {
        if let Some(&(_, part_value)) = part.iter().find(|&&(item, _)| item == rule.item.chars().next().unwrap()) {
            let condition_met = match rule.operator {
                '>' => part_value > rule.value as usize,
                '<' => part_value < rule.value as usize,
                _ => false, // Handle invalid operator
            };

            if condition_met {
                return rule.action.clone();
            }
        }
    }
    workflow.default.clone()
}

fn calculate_accepted_combinations(workflow_name: &str, workflows: &HashMap<String, WorkFlow>, mut ranges: HashMap<char, (usize, usize)>) -> usize {
    match workflow_name {
        "R" => 0,
        "A" => ranges.values().map(|&(lo, hi)| hi - lo + 1).product(),
        _ => {
            let workflow = &workflows[workflow_name];
            let mut total = 0;
            for rule in &workflow.rules {
                let (lo, hi) = ranges[&rule.item.chars().next().unwrap()];
                let (t_range, f_range) = if rule.operator == '<' {
                    ((lo, usize::min(rule.value - 1, hi)), (usize::max(rule.value, lo), hi))
                } else {
                    ((usize::max(rule.value + 1, lo), hi), (lo, usize::min(rule.value, hi)))
                };

                if t_range.0 <= t_range.1 {
                    let mut copy = ranges.clone();
                    copy.insert(rule.item.parse().unwrap(), t_range);
                    total += calculate_accepted_combinations(&rule.action,  workflows, copy);
                }
                if f_range.0 <= f_range.1 {
                    ranges.insert(rule.item.parse().unwrap(), f_range);
                } else {
                    break;
                }
            }
            total + calculate_accepted_combinations(&workflow.default,  workflows, ranges)
        }
    }
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
    fn test_process_parts_through_workflows() {
        let filename = "test.txt";
        let (workflows, parts) = load_data(filename);
        let expected = vec![('A', 7540), ('R', 4286), ('A', 4623), ('R', 4557), ('A', 6951)];
        let results = process_parts_through_workflows(&workflows, &parts);
        assert_eq!(results, expected);
    }

    #[test]
    fn test_process_part_through_workflows() {
        let filename = "test.txt";
        let (workflows, parts) = load_data(filename);
        let expected = ('R', 4286);
        let results = process_part_through_workflows("in", &workflows, &parts[1]);
        assert_eq!(results, expected);
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


}