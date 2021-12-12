use std::collections::{HashMap, HashSet};

use crate::input_const;

pub fn solution1() -> usize {
    Graph::from_str(input_const!("12")).solution1()
}

struct Graph {
    nodes: HashSet<String>,
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn from_str(s: impl AsRef<str>) -> Self {
        let mut nodes = HashSet::new();
        let mut edges = HashMap::new();
        for line in s.as_ref().lines().filter(|s| !s.is_empty()) {
            let mut parts = line.split("-");
            let n1 = parts.next().unwrap();
            let n2 = parts.next().unwrap();

            nodes.insert(n1.to_string());
            nodes.insert(n2.to_string());

            edges
                .entry(n1.to_string())
                .or_insert(HashSet::new())
                .insert(n2.to_string());
            edges
                .entry(n2.to_string())
                .or_insert(HashSet::new())
                .insert(n1.to_string());
        }

        Self { nodes, edges }
    }

    
    fn solution1(&mut self) -> usize {
        self.list_paths().len()
    } 

    fn list_paths(&mut self) -> Vec<String> {
        self.list_paths_impl("start".into(), HashSet::from(["start".to_string()]))
    }

    fn list_paths_impl(&mut self, start: String, visited: HashSet<String>) -> Vec<String> {
        if start == "end" {
            return vec!["end".to_string()];
        }
        let mut next_nodes = HashSet::new();
        let all_nodes = self.edges.get(&start).unwrap().clone();

        for node in all_nodes {
            if is_large(&node) || !visited.contains(&node) {
                next_nodes.insert(node);
            }
        }

        let mut result = vec![];

        for node in next_nodes {
            let mut new_visited = visited.clone();
            new_visited.insert(node.to_string());

            let paths = self.list_paths_impl(node.to_string(), new_visited);
            result.extend(paths.into_iter().map(|s| format!("{},{}", start, s)));
        }

        result
    }
}

fn is_large(s: &str) -> bool {
    s.chars().next().unwrap().is_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT: &str = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX

pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;

    #[test]
    fn given_example() {
        let mut graph = Graph::from_str(GIVEN_INPUT);
        assert_eq!(graph.solution1(), 226);
    }

    #[test]
    fn test_is_large() {
        assert_eq!(is_large("asdf"), false);
        assert_eq!(is_large("AS"), true);
    }

    #[test]
    fn parse_grid() {
        let mut grid = Graph::from_str(
            r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#,
        );

        assert_eq!(
            grid.edges.get("A").unwrap(),
            &HashSet::from([
                "start".to_string(),
                "c".to_string(),
                "b".to_string(),
                "end".to_string(),
            ])
        );
        assert_eq!(
            grid.edges.get("c").unwrap(),
            &HashSet::from(["A".to_string()])
        );
        let mut graph= Graph::from_str("start-A\nA-end\nb-end\nA-b");
        assert_eq!(graph.edges.get("A").unwrap(), &HashSet::from(["start".to_string(), "b".to_string(), "end".to_string()]));

        dbg!(grid.list_paths());
        assert_eq!(grid.solution1(), 10);
    }

    #[test]
    fn trivial_examples() {
        assert_eq!(Graph::from_str("start-end").solution1(), 1);
        assert_eq!(Graph::from_str("start-a\na-end").solution1(), 1);
        assert_eq!(Graph::from_str("start-A\nA-end\nb-end\nA-b").solution1(), 3);
    }
}
