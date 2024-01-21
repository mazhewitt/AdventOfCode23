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
impl ModuleType {
    // Method to handle receiving a pulse
    pub fn receive_pulse(&mut self, pulse: Pulse) -> Vec<(NodeIndex, Pulse)> {
        // Initialize an empty vector to collect any outbound pulses
        let mut outbound_pulses = Vec::new();

        match self {
            ModuleType::FlipFlop(state) => {
                // Handle flip-flop logic
            },
            ModuleType::Conjunction(receive_node_map) => {
                // Handle conjunction logic
            },

            ModuleType::Broadcaster => {
                // Handle broadcaster logic
            },

        }

        outbound_pulses // Return any generated outbound pulses
    }
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

        println!("{:?}", graph);
    }


    #[test]
    fn flip_flop_receives_high_pulse() {
        let mut flip_flop = ModuleType::FlipFlop(false);
        let outbound_pulses = flip_flop.receive_pulse(Pulse::High);
        assert_eq!(outbound_pulses.len(), 0, "Flip-flop should not generate any outbound pulses when receiving a high pulse.");
        assert_eq!(flip_flop, ModuleType::FlipFlop(false), "Flip-flop should be off after receiving a high pulse.");
        let mut flip_flop2 = ModuleType::FlipFlop(true);
        let outbound_pulses = flip_flop2.receive_pulse(Pulse::High);
        assert_eq!(outbound_pulses.len(), 0, "Flip-flop should not generate any outbound pulses when receiving a high pulse.");
        assert_eq!(flip_flop2, ModuleType::FlipFlop(true), "Flip-flop should be off after receiving a high pulse.");

    }

    #[test]
    fn flip_flop_receives_low_pulse() {
        let mut flip_flop = ModuleType::FlipFlop(false);
        let outbound_pulses = flip_flop.receive_pulse(Pulse::Low);
        assert_eq!(outbound_pulses.len(), 1, "Off Flip-flop should generate a high pulse when receiving a low pulse.");
        assert_eq!(flip_flop, ModuleType::FlipFlop(true), "Off Flip-flop should be on after receiving a high pulse.");
        assert_eq!(outbound_pulses[0], (NodeIndex::new(0), Pulse::High), "Off flip-flop should generate a high pulse when receiving a low pulse.");

        let mut flip_flop2 = ModuleType::FlipFlop(true);
        let outbound_pulses = flip_flop2.receive_pulse(Pulse::Low);
        assert_eq!(outbound_pulses.len(), 1, "On Flip-flop should generate a low pulse when receiving a low pulse.");
        assert_eq!(flip_flop2, ModuleType::FlipFlop(false), "Flip-flop should be off after receiving a low pulse.");
        assert_eq!(outbound_pulses[0], (NodeIndex::new(0), Pulse::Low), "On flip-flop should generate a low pulse when receiving a low pulse.");

    }

}