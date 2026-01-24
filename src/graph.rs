///权重
#[derive(Debug,Clone,Copy)]
pub enum Weight {
    ///无穷大
    Infinity, 
    ///权值
    Value(i32), 
}
///邻接矩阵表示图
#[derive(Debug,Clone)]
pub struct MGraph<T>{
    pub vertex:Vec<T>, //顶点表
    pub edge:Vec<Vec<Weight>>, //邻接矩阵
    pub arcnum:u32, //边数
}

impl<T> MGraph<T> {
    ///This function creates an empty graph and needs to fill in the data by itself. 
    /// 
    /// A complete encapsulation function has not been designed for the time being, 
    /// 
    /// and the data needs to be carefully checked.
    pub fn new()->Self{
        Self { vertex:Vec::new(), edge:Vec::new(), arcnum:0 }
    }
    ///添加新的顶点
    pub fn add_vex(&mut self,vex:T){
        self.vertex.push(vex);
    }
}
impl<T> FirstNeighbor for MGraph<T> {
    type VertexId = usize;
    fn first_neighbor(&self,x:Self::VertexId)->Option<Self::VertexId> {
        let row=self.edge.get(x)?;
        for (i,weight) in row.iter().enumerate() {
            if let Weight::Value(_) = weight {
                return Some(i)
            }
        }
        None
    }
}
impl<T> NextNeighbor for MGraph<T> {
    fn next_neighbor(&self, x: Self::VertexId,y:Self::VertexId) -> Option<Self::VertexId> {
        let row=self.edge.get(x)?;
        for (i,weight) in row.iter().enumerate().skip(y+1) {
            if let Weight::Value(_)=weight{
                return Some(i)
            }
        }
        None
    }
}
///求图顶点x的第一个邻接点
pub trait FirstNeighbor {
    type VertexId;
    fn first_neighbor(&self,x:Self::VertexId)->Option<Self::VertexId>;
}
///求图顶点x除了y以为的第一个邻接点，y为x一个邻接点
pub trait NextNeighbor:FirstNeighbor {
    fn next_neighbor(&self, x: Self::VertexId,y:Self::VertexId) -> Option<Self::VertexId>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weight_enum() {
        let infinity = Weight::Infinity;
        let value = Weight::Value(10);

        match infinity {
            Weight::Infinity => assert!(true),
            _ => panic!("Expected Infinity"),
        }

        match value {
            Weight::Value(v) => assert_eq!(v, 10),
            _ => panic!("Expected Value(10)"),
        }
    }

    #[test]
    fn test_mgraph_creation() {
        let graph: MGraph<i32> = MGraph::new();
        
        assert_eq!(graph.vertex.len(), 0);
        assert_eq!(graph.arcnum, 0);
        assert_eq!(graph.edge.len(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut graph: MGraph<String> = MGraph::new();
        
        graph.add_vex("A".to_string());
        graph.add_vex("B".to_string());
        
        assert_eq!(graph.vertex.len(), 2);
        assert_eq!(graph.vertex[0], "A");
        assert_eq!(graph.vertex[1], "B");
    }

    #[test]
    fn test_first_neighbor_with_empty_graph() {
        let graph: MGraph<i32> = MGraph::new();
        
        // Should return None for an empty graph
        let result = graph.first_neighbor(0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_first_neighbor_with_no_edges() {
        let mut graph: MGraph<&str> = MGraph::new();
        graph.add_vex("A");
        graph.add_vex("B");
        // Initialize adjacency matrix with infinity values
        graph.edge = vec![
            vec![Weight::Infinity, Weight::Infinity],
            vec![Weight::Infinity, Weight::Infinity]
        ];
        
        let result = graph.first_neighbor(0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_first_neighbor_with_edge() {
        let mut graph: MGraph<&str> = MGraph::new();
        graph.add_vex("A");
        graph.add_vex("B");
        graph.add_vex("C");
        // Initialize adjacency matrix with some edges
        graph.edge = vec![
            vec![Weight::Infinity, Weight::Value(5), Weight::Infinity],
            vec![Weight::Value(3), Weight::Infinity, Weight::Value(2)],
            vec![Weight::Infinity, Weight::Value(1), Weight::Infinity]
        ];
        
        // First neighbor of A (index 0) should be B (index 1)
        let result = graph.first_neighbor(0);
        assert_eq!(result, Some(1));
        
        // First neighbor of B (index 1) should be A (index 0)
        let result = graph.first_neighbor(1);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_next_neighbor() {
        let mut graph: MGraph<&str> = MGraph::new();
        graph.add_vex("A");
        graph.add_vex("B");
        graph.add_vex("C");
        graph.add_vex("D");
        // Initialize adjacency matrix with some edges
        graph.edge = vec![
            vec![Weight::Infinity, Weight::Value(5), Weight::Value(3), Weight::Value(7)],
            vec![Weight::Value(3), Weight::Infinity, Weight::Value(2), Weight::Infinity],
            vec![Weight::Value(4), Weight::Value(1), Weight::Infinity, Weight::Value(6)],
            vec![Weight::Value(2), Weight::Infinity, Weight::Value(1), Weight::Infinity]
        ];
        
        // For A (index 0), neighbors are B(1), C(2), D(3)
        // First neighbor is B(1), next neighbor after B should be C(2)
        let result = graph.next_neighbor(0, 1);
        assert_eq!(result, Some(2));
        
        // Next neighbor after C should be D
        let result = graph.next_neighbor(0, 2);
        assert_eq!(result, Some(3));
        
        // After D there are no more neighbors
        let result = graph.next_neighbor(0, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_trait_implementations() {
        let mut graph: MGraph<i32> = MGraph::new();
        graph.add_vex(1);
        graph.add_vex(2);
        graph.edge = vec![
            vec![Weight::Infinity, Weight::Value(10)],
            vec![Weight::Value(10), Weight::Infinity]
        ];
        
        // Test that the implementations satisfy the traits
        let first_neighbor = graph.first_neighbor(0);
        assert_eq!(first_neighbor, Some(1));
        
        let next_neighbor = graph.next_neighbor(0, 1);
        assert_eq!(next_neighbor, None); // There is only one neighbor
    }
}