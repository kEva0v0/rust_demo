use graph;

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
        self.adjacency_table
            .entry(edge.1.to_string())
            .and_modify(|e| {
                e.push((edge.0.to_string(), edge.2));
            });
    }
}
