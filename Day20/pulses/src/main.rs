use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;


fn main() {
    println!("Hello, world!");
}
#[derive(PartialEq, Eq, Debug)]
enum ModuleType {
    FlipFlop(bool), // bool represents the state: on or off
    Conjunction(HashMap<NodeIndex, bool>), // Tracks the most recent pulse for each input
    Broadcaster,
}


// Define a pulse type
enum Pulse {
    High,
    Low,
}

fn simulate_pulse(graph: &mut DiGraph<ModuleType, ()>, start_node: NodeIndex, pulse: Pulse) {

}

fn build_graph_from_input(input: &str) -> DiGraph<ModuleType, ()> {
    let mut graph = DiGraph::<ModuleType, ()>::new();
    let mut node_indices = HashMap::new(); // This will map module names (String) to NodeIndex

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts.len() != 2 {
            continue;
        }

        let module_info = parts[0];
        let destinations = parts[1].split(", ").collect::<Vec<&str>>();

        // Determine the type of the module
        let (module_type, module_name) = parse_module_info(module_info);

        // Get or create the node index for the module using its name (not ModuleType)
        let node_index = *node_indices
            .entry(module_name.to_string())
            .or_insert_with(|| graph.add_node(module_type));

        // Connect the module to its destinations
        for dest in destinations {
            let dest_index = *node_indices
                .entry(dest.to_string())
                .or_insert_with(|| graph.add_node(ModuleType::FlipFlop(false))); // Default type, might need to adjust based on your needs
            graph.add_edge(node_index, dest_index, ());
        }
    }

    graph
}


// Helper function to parse module info and determine its type
fn parse_module_info(info: &str) -> (ModuleType, &str) {
    if info.starts_with('%') {
        (ModuleType::FlipFlop(false), &info[1..])
    } else if info.starts_with('&') {
        (ModuleType::Conjunction(HashMap::new()), &info[1..])
    } else {
        (ModuleType::Broadcaster, info) // assuming all others are broadcasters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_building() {
        let input = "\
            broadcaster -> a, b, c\n\
            %a -> b\n\
            %b -> c\n\
            %c -> inv\n\
            &inv -> a\
        ";

        let graph = build_graph_from_input(input);

        // Verify that the graph has the correct number of nodes and edges
        assert_eq!(graph.node_count(), 5, "Graph does not contain 5 nodes.");
        assert_eq!(graph.edge_count(), 7, "Graph does not contain 7 edges.");
    }
}