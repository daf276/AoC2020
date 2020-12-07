use petgraph::algo::has_path_connecting;
use petgraph::prelude::*;
use petgraph::visit::Dfs;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::net::Incoming;
use std::str::EncodeUtf16;

lazy_static! {
    static ref BAG_RE: Regex = Regex::new(r"(\w+\s\w+)\sbags\scontain").unwrap();
    static ref CONTENT_RE: Regex = Regex::new(r"\s(\d)\s(\w+\s\w+)\sbags?[,.]").unwrap();
}

pub fn read_input() -> (Graph<String, u32>, HashMap<String, NodeIndex>) {
    let mut starting_nodes: HashMap<String, NodeIndex> = HashMap::new();
    let mut graph = Graph::<String, u32>::new();

    let lines = include_str!("../input/day7").lines();

    for line in lines.clone() {
        let starting_bag = BAG_RE.captures(line).unwrap()[1].to_string();
        let bag_node = graph.add_node(starting_bag.clone());

        starting_nodes.insert(starting_bag.clone(), bag_node);
    }

    for line in lines {
        let starting_bag = BAG_RE.captures(line).unwrap()[1].to_string();

        for content in CONTENT_RE.captures_iter(line) {
            let bag = content[2].to_string();
            let weight = content[1].parse::<u32>().unwrap();
            graph.add_edge(starting_nodes[&starting_bag], starting_nodes[&bag], weight);
        }
    }

    return (graph, starting_nodes);
}

pub fn part1(mut gr: &Graph<String, u32>) -> usize {
    let mut bags = HashSet::new();
    let shiny_gold = gr.node_indices().find(|i| gr[*i] == "shiny gold").unwrap();
    traverse_bags(gr, shiny_gold, &mut bags);
    return bags.len();
}

fn traverse_bags(mut gr: &Graph<String, u32>, node: NodeIndex, bags: &mut HashSet<NodeIndex>) {
    let mut walker = gr.neighbors_directed(node, Incoming).detach();
    while let Some((_, node)) = walker.next(&gr) {
        bags.insert(node);
        traverse_bags(gr, node, bags);
    }
}

pub fn part2(mut gr: &Graph<String, u32>) -> u32 {
    let shiny_gold = gr.node_indices().find(|i| gr[*i] == "shiny gold").unwrap();
    return count_bags(gr, shiny_gold);
}

fn count_bags(mut gr: &Graph<String, u32>, node: NodeIndex) -> u32 {
    return if gr.neighbors_directed(node, Outgoing).next() == None {
        0
    } else {
        let mut bag_count = 0;
        let mut walker = gr.neighbors_directed(node, Outgoing).detach();
        while let Some((edge, node)) = walker.next(&gr) {
            bag_count += (gr[edge] * count_bags(gr, node)) + gr[edge];
        }
        bag_count
    };
}
