use graph;

pub struct DirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for DirectedGraph {
    fn new() -> DirectedGraph {
        DirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
}