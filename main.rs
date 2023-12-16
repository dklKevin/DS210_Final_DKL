use petgraph::graph::{UnGraph, NodeIndex};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Text File Read
    let graph_path = Path::new("/Users/kyulee/Downloads/email-Eu-core.txt");
    let file = File::open(&graph_path)?;
    let reader = io::BufReader::new(file);

    // Undirected Graph Creation
    let mut graph: UnGraph<usize, ()> = UnGraph::new_undirected();

    // Read each line and add edges to the graph
    for line in reader.lines() {
        let line = line?;
        let nodes: Vec<usize> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nodes.len() == 2 {
            let node1 = NodeIndex::new(nodes[0]);
            let node2 = NodeIndex::new(nodes[1]);

            if graph.node_weight(node1).is_none() {
                graph.add_node(nodes[0]);
            }
            if graph.node_weight(node2).is_none() {
                graph.add_node(nodes[1]);
            }

            graph.add_edge(node1, node2, ());
        }
    }

    // Average distance between all pairs of nodes calculation
    let mut total_distance_all = 0;
    let mut count_all = 0;

    for start_node in graph.node_indices() {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_node, 0));

        while let Some((node, distance)) = queue.pop_front() {
            if !visited.insert(node) {
                continue;
            }

            // Had to exclude the distance to the node itself
            if node != start_node {
                total_distance_all += distance;
                count_all += 1;
            }

            for neighbor in graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }
    }

    let average_distance_all = total_distance_all as f64 / count_all as f64;
    println!("Average distance between all pairs of Vertices: {}", average_distance_all);

    // Text File Read pt2
    let department_path = Path::new("/Users/kyulee/Downloads/email-Eu-core-department-labels.txt");
    let file = File::open(&department_path)?;
    let reader = io::BufReader::new(file);

    // Map of professors to departments creation
    let mut department_map: HashMap<usize, usize> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<usize> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if parts.len() == 2 {
            let professor_id = parts[0];
            let department_id = parts[1];
            department_map.insert(professor_id, department_id);
        }
    }

    // Average distance for each department calculation
    let mut department_distances: HashMap<usize, f64> = HashMap::new();
    let unique_departments: HashSet<_> = department_map.values().cloned().collect();
    let mut lowest_avg_dept = 0;
    let mut lowest_avg_distance = f64::MAX;
    let mut highest_avg_dept = 0;
    let mut highest_avg_distance = 0.0;
    

    for department_id in unique_departments {

        let department_graph = graph
            .node_indices()
            .filter(|&node| department_map.get(&node.index()) == Some(&department_id))
            .collect::<HashSet<_>>();

        let mut total_distance_dept = 0;
        let mut count_dept = 0;

        for &start_node in &department_graph {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((start_node, 0)); // Push the starting node with depth 0
        
            while let Some((node, depth)) = queue.pop_front() {
                if !visited.insert(node) {
                    continue;
                }
        
                total_distance_dept += depth;
                count_dept += 1;
        
                for neighbor in graph.neighbors(node) {
                    if !visited.contains(&neighbor) {
                        queue.push_back((neighbor, depth + 1)); // Increment depth for neighbors
                    }
                }
            }
        }        

        let average_distance_dept = if count_dept > 0 {
            total_distance_dept as f64 / count_dept as f64
        } else {
            0.0
        };

        department_distances.insert(department_id, average_distance_dept);

        if average_distance_dept < lowest_avg_distance {
            lowest_avg_distance = average_distance_dept;
            lowest_avg_dept = department_id;
        }

        if average_distance_dept > highest_avg_distance {
            highest_avg_distance = average_distance_dept;
            highest_avg_dept = department_id;
        }

        println!("Average distance between vertices in department {}: {}", department_id, average_distance_dept);
    }

    println!("Lowest Average Distance between Vertices: {} (Department {})", lowest_avg_distance, lowest_avg_dept);
    println!("Highest Average Distance between Vertices: {} (Department {})", highest_avg_distance, highest_avg_dept);

    Ok(())
}
#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::UnGraph;
    use std::collections::{HashSet, VecDeque};

    // Small test graph creation
    fn create_test_graph() -> UnGraph<usize, ()> {
        let mut graph = UnGraph::<usize, ()>::new_undirected();
        let node_indices: Vec<_> = (0..5).map(|n| graph.add_node(n)).collect();
        graph.add_edge(node_indices[0], node_indices[1], ());
        graph.add_edge(node_indices[1], node_indices[2], ());
        graph.add_edge(node_indices[2], node_indices[3], ());
        graph.add_edge(node_indices[3], node_indices[4], ());
        graph.add_edge(node_indices[4], node_indices[0], ());

        graph
    }

    // BFS Test
    #[test]
    fn test_bfs_functionality() {
        let graph = create_test_graph();
        let start_node = NodeIndex::new(0);
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_node, 0));

        while let Some((node, distance)) = queue.pop_front() {
            if !visited.insert(node) {
                continue;
            }

            distances.insert(node.index(), distance);

            for neighbor in graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }

        // Expected distances for each node from the start node
        let expected_distances = vec![0, 1, 2, 2, 1]; // Distance from each node, will explain more in report.

        for (index, &expected_distance) in expected_distances.iter().enumerate() {
            assert_eq!(
                *distances.get(&index).unwrap_or(&usize::MAX),
                expected_distance,
                "Distance from node 0 to {} is incorrect",
                index
            );
        }
    }

    #[test]
    fn test_graph_construction() {
        let manually_created_graph = create_test_graph();
    
        // Logic replication test to check BFS is working properly
        let mut function_created_graph = UnGraph::<usize, ()>::new_undirected();
        let node_indices: Vec<_> = (0..5).map(|n| function_created_graph.add_node(n)).collect();
        function_created_graph.add_edge(node_indices[0], node_indices[1], ());
        function_created_graph.add_edge(node_indices[1], node_indices[2], ());
        function_created_graph.add_edge(node_indices[2], node_indices[3], ());
        function_created_graph.add_edge(node_indices[3], node_indices[4], ());
        function_created_graph.add_edge(node_indices[4], node_indices[0], ());
    
        assert_eq!(manually_created_graph.node_count(), function_created_graph.node_count());
        assert_eq!(manually_created_graph.edge_count(), function_created_graph.edge_count());
    }
}
