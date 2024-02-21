use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use petgraph::Direction;

fn main() {
    println!("Hello, world!");
}

pub trait Module {
    fn receive_pulse(&mut self, pulse: Pulse, from: &str) -> Option<Pulse>;
    fn get_name(&self) -> String;

    fn get_module_type(&self) -> ModuleType;
}

// Define a pulse type
pub enum Pulse {
    High,
    Low,
}

pub enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

// Flip-flop module
pub struct FlipFlop {
    is_on: bool,
    name: String,
}

impl Module for FlipFlop {
    fn receive_pulse(&mut self, pulse: Pulse, from: &str) -> Option<Pulse>{
        match pulse {
            Pulse::High => {
                 None
            },
            Pulse::Low => {
                self.is_on = !self.is_on;
                if self.is_on {
                    Some(Pulse::High)
                }
                else {
                    Some(Pulse::Low)
                }
            },
        }

    }

    fn get_name(&self) -> String {
        self.name.clone()
    }


    fn get_module_type(&self) -> ModuleType {
        ModuleType::FlipFlop
    }
}

// Conjunction module
pub struct Conjunction {
    input_states: HashMap<String, Pulse>,
    name: String,
}

impl Module for Conjunction {
    fn receive_pulse(&mut self, pulse: Pulse, from: &str) -> Option<Pulse>{
        self.input_states.insert(from.to_string(), pulse);
        if self.input_states.values().all(|x| x == Pulse::High) {
            Some(Pulse::High)
        }
        else {
            Some(Pulse::Low)
        }
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_module_type(&self) -> ModuleType {
        ModuleType::Conjunction
    }
}

pub struct Broadcaster {
    name: String,
}

impl Module for Broadcaster {
    fn receive_pulse(&mut self, pulse: Pulse, from: &str)-> Option<Pulse>{
        Some(pulse)
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_module_type(&self) -> ModuleType {
        ModuleType::Broadcaster
    }
}



fn build_graph_from_input(input: &str) -> DiGraph<Box<dyn Module>, ()> {
    let mut graph: DiGraph<Box<dyn Module>, ()> = DiGraph::new();
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();

    // loop through each line in the input build the nodes
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts.len() != 2 {
            continue; // Skip invalid lines
        }

        let src = parts[0].trim();
        let dst_list = parts[1].split(", ").collect::<Vec<&str>>();
        if src.starts_with("%") {
            // This is a conjunction
            let conjunction_name = src[1..].to_string();
            let conjunction = Conjunction {
                input_states: HashMap::new(),
                name: conjunction_name.clone(),
            };
            nodes.insert(conjunction_name, graph.add_node(Box::new(conjunction)));
        }
        else if src.starts_with("&") {
            // This is a broadcaster
            let broadcaster_name = src[1..].to_string();
            let broadcaster = Broadcaster {
                name: broadcaster_name.clone(),
            };
            let broadcaster_node = graph.add_node(Box::new(broadcaster));
            nodes.insert(broadcaster_name, broadcaster_node);
        }
        else {
            // This is a flip-flop
            let flip_flop_name = src.to_string();
            let flip_flop = FlipFlop {
                is_on: false,
                name: flip_flop_name.clone(),
            };
            let flip_flop_node = graph.add_node(Box::new(flip_flop));
            nodes.insert(flip_flop_name, flip_flop_node);
        }

    }
    // loop through each line in the input build the edges
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts.len() != 2 {
            continue; // Skip invalid lines
        }

        let src = parts[0].trim();
        let dst_list = parts[1].split(", ").collect::<Vec<&str>>();
        let src_idx = *nodes.entry(src[1..].to_string()).get().unwrap();
        for dst in dst_list {
            // add the edge
            let dst_idx = *nodes.entry(dst.to_string()).get().unwrap();
            graph.add_edge(src_idx, dst_idx, ());
            // prime the conjunction with a low from the source
            if graph[dst_idx].get_module_type() == ModuleType::Conjunction {
                // send low pulse from src
                graph[dst_idx].receive_pulse(Pulse::Low, &graph[src_idx].get_name());
            }
        }

    }


    graph
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


    #[test]
    fn flip_flop_receives_high_pulse() {
        let mut flip_flop = FlipFlop { is_on: false, name: "test".to_string() };
        let outbound_pulse = flip_flop.receive_pulse(Pulse::High, &"test2".to_string());
        assert_eq!(outbound_pulse, Option::None, "Flip-flop should not generate any outbound pulses when receiving a high pulse.");
        assert_eq!(flip_flop.is_on, false, "Flip-flop should be off after receiving a high pulse.");
        let mut flip_flop2 = FlipFlop { is_on: true, name: "test".to_string() };
        let outbound_pulse2 = flip_flop2.receive_pulse(Pulse::High, &"test2".to_string());
        assert_eq!(outbound_pulse2, None, "Flip-flop should not generate any outbound pulses when receiving a high pulse.");
        assert_eq!(flip_flop2.is_on, false, "Flip-flop should be off after receiving a high pulse.");

    }

    #[test]
    fn flip_flop_receives_low_pulse() {
        let mut flip_flop = FlipFlop { is_on: false, name: "test".to_string() };
        let outbound_pulse = flip_flop.receive_pulse(Pulse::Low, &"test".to_string());
        assert_eq!(outbound_pulse, Some(Pulse::High), "Off Flip-flop should generate a high pulse when receiving a low pulse.");
        assert_eq!(flip_flop.is_on, true, "Off Flip-flop should be on after receiving a high pulse.");

        let mut flip_flop2 = FlipFlop { is_on: true, name: "test".to_string() };
        let outbound_pulse2 = flip_flop2.receive_pulse(Pulse::Low, &"test".to_string());
        assert_eq!(outbound_pulse2, Some(Pulse::Low), "On Flip-flop should generate a low pulse when receiving a low pulse.");
        assert_eq!(flip_flop2.is_on, false, "Flip-flop should be off after receiving a low pulse.");

    }

}