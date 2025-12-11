use std::collections::HashMap;

fn find_all_paths(
    current: &str,
    target: &str,
    graph: &HashMap<String, Vec<String>>,
    mut path: Vec<String>,
) -> Vec<Vec<String>> {
    path.push(current.to_string());

    if current == target {
        return vec![path];
    }

    if let Some(outputs) = graph.get(current) {
        let mut all_paths = vec![];
        for output in outputs {
            let paths = find_all_paths(output, target, graph, path.clone());
            all_paths.extend(paths);
        }
        all_paths
    } else {
        vec![]
    }
}

fn main() {
    let data = include_str!("../input.txt");

    let graph = data
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.trim().split(':').collect();
            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let outputs: Vec<String> = parts[1]
                    .trim()
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect();
                Some((name, outputs))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    let paths = find_all_paths("you", "out", &graph, vec![]);
    println!("Number of paths: {}", paths.len());
}
