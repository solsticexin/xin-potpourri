use std::collections::VecDeque;
use std::collections::HashSet;
use std::hash::Hash;

/// 1. 定义图的基本行为
/// 这里的 VertexId 不再是 T，而是代表位置的“索引”
pub trait GraphOps {
    // 约束：ID 必须轻量 (Copy) 且能作为哈希键 (Eq + Hash)
    // 对于 MGraph，这个类型就是 usize
    type VertexId: Copy + Eq + Hash + From<usize> + Into<usize>;

    // 算法需要知道图有多大，好创建 visited 数组
    fn vertex_count(&self) -> usize;

    // 找第一个邻居的 ID
    fn first_neighbor(&self, x: Self::VertexId) -> Option<Self::VertexId>;

    // 找下一个邻居的 ID
    fn next_neighbor(&self, x: Self::VertexId, y: Self::VertexId) -> Option<Self::VertexId>;
}

/// 2. 通用 BFS 实现
/// 只要实现了 GraphOps，就能自动获得 BFS 能力
pub trait BFS: GraphOps {
    fn bfs_traverse<F>(&self, start: Self::VertexId, mut visit: F)
    where
        F: FnMut(Self::VertexId), // 闭包接收的是 ID
    {
        // 如果是 usize 类型的 ID，我们可以用更快的 Vec<bool>，
        // 但为了演示通用性，这里用 HashSet
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visit(start);
        visited.insert(start);
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            let mut w_opt = self.first_neighbor(u);
            
            while let Some(w) = w_opt {
                if !visited.contains(&w) {
                    visit(w);
                    visited.insert(w);
                    queue.push_back(w);
                }
                w_opt = self.next_neighbor(u, w);
            }
        }
    }
}

// 为所有满足条件的类型自动实现 BFS
impl<G: GraphOps> BFS for G {}


#[derive(Debug, Clone, Copy, PartialEq)] // 添加 PartialEq 以便比较
pub enum Weight {
    Infinity,
    Value(i32),
}

#[derive(Debug, Clone)]
pub struct MGraph<T> {
    pub vertex: Vec<T>,          // 这里存复杂的 T
    pub edge: Vec<Vec<Weight>>,  // 这里只存关系
}

impl<T> MGraph<T> {
    pub fn new() -> Self {
        Self { vertex: Vec::new(), edge: Vec::new() }
    }

    // 添加节点，返回它的 ID (usize)
    pub fn add_node(&mut self, data: T) -> usize {
        let id = self.vertex.len();
        self.vertex.push(data);
        
        // 扩容矩阵 (简化版：每次加点都重构一下矩阵大小，实际应用要优化)
        // 保证矩阵是 N x N
        for row in &mut self.edge {
            row.push(Weight::Infinity);
        }
        self.edge.push(vec![Weight::Infinity; id + 1]);
        
        id // 把 ID 给用户，就像给了个取票凭证
    }

    // 添加边 (通过 ID 操作)
    pub fn add_edge(&mut self, start: usize, end: usize, weight: i32) {
        if start < self.vertex.len() && end < self.vertex.len() {
            self.edge[start][end] = Weight::Value(weight);
            // 如果是无向图，还得加 self.edge[end][start] = ...
        }
    }
    
    // 辅助函数：通过 ID 拿数据
    pub fn get_data(&self, id: usize) -> Option<&T> {
        self.vertex.get(id)
    }
}

impl<T> GraphOps for MGraph<T> {
    type VertexId = usize; // 核心：ID 就是下标

    fn vertex_count(&self) -> usize {
        self.vertex.len()
    }

    fn first_neighbor(&self, x: Self::VertexId) -> Option<Self::VertexId> {
        // x 是 usize，直接用作数组下标，无需任何转换，O(1) 复杂度
        if let Some(row) = self.edge.get(x) {
            for (i, weight) in row.iter().enumerate() {
                if let Weight::Value(_) = weight {
                    return Some(i); // 返回找到的邻居下标
                }
            }
        }
        None
    }

    fn next_neighbor(&self, x: Self::VertexId, y: Self::VertexId) -> Option<Self::VertexId> {
        if let Some(row) = self.edge.get(x) {
            // 从 y+1 开始找
            for (i, weight) in row.iter().enumerate().skip(y + 1) {
                if let Weight::Value(_) = weight {
                    return Some(i);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mgraph_add_node() {
        let mut graph = MGraph::new();
        
        let id1 = graph.add_node("A");
        let id2 = graph.add_node("B");
        let id3 = graph.add_node("C");
        
        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(id3, 2);
        assert_eq!(graph.vertex_count(), 3);
    }

    #[test]
    fn test_mgraph_get_data() {
        let mut graph = MGraph::new();
        
        let id1 = graph.add_node("Node1");
        let id2 = graph.add_node("Node2");
        
        assert_eq!(graph.get_data(id1), Some(&"Node1"));
        assert_eq!(graph.get_data(id2), Some(&"Node2"));
        assert_eq!(graph.get_data(999), None);
    }

    #[test]
    fn test_mgraph_add_edge() {
        let mut graph = MGraph::new();
        
        let id1 = graph.add_node("A");
        let id2 = graph.add_node("B");
        let id3 = graph.add_node("C");
        
        graph.add_edge(id1, id2, 10);
        graph.add_edge(id1, id3, 20);
        graph.add_edge(id2, id3, 5);
        
        assert_eq!(graph.edge[id1][id2], Weight::Value(10));
        assert_eq!(graph.edge[id1][id3], Weight::Value(20));
        assert_eq!(graph.edge[id2][id3], Weight::Value(5));
        assert_eq!(graph.edge[id2][id1], Weight::Infinity);
    }

    #[test]
    fn test_graphops_vertex_count() {
        let mut graph = MGraph::new();
        
        assert_eq!(graph.vertex_count(), 0);
        
        graph.add_node("A");
        graph.add_node("B");
        
        assert_eq!(graph.vertex_count(), 2);
    }

    #[test]
    fn test_graphops_first_neighbor() {
        let mut graph = MGraph::new();
        
        let id1 = graph.add_node("A");
        let id2 = graph.add_node("B");
        let id3 = graph.add_node("C");
        
        graph.add_edge(id1, id2, 10);
        graph.add_edge(id1, id3, 20);
        
        assert_eq!(graph.first_neighbor(id1), Some(id2));
        assert_eq!(graph.first_neighbor(id2), None);
    }

    #[test]
    fn test_graphops_next_neighbor() {
        let mut graph = MGraph::new();
        
        let id1 = graph.add_node("A");
        let id2 = graph.add_node("B");
        let id3 = graph.add_node("C");
        let id4 = graph.add_node("D");
        
        graph.add_edge(id1, id2, 10);
        graph.add_edge(id1, id3, 20);
        graph.add_edge(id1, id4, 30);
        
        let first = graph.first_neighbor(id1).unwrap();
        assert_eq!(first, id2);
        
        let second = graph.next_neighbor(id1, first).unwrap();
        assert_eq!(second, id3);
        
        let third = graph.next_neighbor(id1, second).unwrap();
        assert_eq!(third, id4);
        
        let fourth = graph.next_neighbor(id1, third);
        assert_eq!(fourth, None);
    }

    #[test]
    fn test_bfs_traverse_simple() {
        let mut graph = MGraph::new();
        
        let id0 = graph.add_node("0");
        let id1 = graph.add_node("1");
        let id2 = graph.add_node("2");
        let id3 = graph.add_node("3");
        
        graph.add_edge(id0, id1, 1);
        graph.add_edge(id0, id2, 1);
        graph.add_edge(id1, id3, 1);
        
        let mut visited_order = Vec::new();
        graph.bfs_traverse(id0, |id| visited_order.push(id));
        
        assert_eq!(visited_order, vec![id0, id1, id2, id3]);
    }

    #[test]
    fn test_bfs_traverse_complex() {
        let mut graph = MGraph::new();
        
        let id0 = graph.add_node("0");
        let id1 = graph.add_node("1");
        let id2 = graph.add_node("2");
        let id3 = graph.add_node("3");
        let id4 = graph.add_node("4");
        
        graph.add_edge(id0, id1, 1);
        graph.add_edge(id0, id2, 1);
        graph.add_edge(id1, id3, 1);
        graph.add_edge(id2, id4, 1);
        
        let mut visited_order = Vec::new();
        graph.bfs_traverse(id0, |id| visited_order.push(id));
        
        assert_eq!(visited_order, vec![id0, id1, id2, id3, id4]);
    }

    #[test]
    fn test_bfs_traverse_with_data_access() {
        let mut graph = MGraph::new();
        
        let id0 = graph.add_node("Start");
        let id1 = graph.add_node("Middle");
        let id2 = graph.add_node("End");
        
        graph.add_edge(id0, id1, 1);
        graph.add_edge(id1, id2, 1);
        
        let mut visited_data = Vec::new();
        graph.bfs_traverse(id0, |id| {
            if let Some(data) = graph.get_data(id) {
                visited_data.push(*data);
            }
        });
        
        assert_eq!(visited_data, vec!["Start", "Middle", "End"]);
    }

    #[test]
    fn test_weight_comparison() {
        assert_eq!(Weight::Value(10), Weight::Value(10));
        assert_ne!(Weight::Value(10), Weight::Value(20));
        assert_ne!(Weight::Value(10), Weight::Infinity);
    }

    #[test]
    fn test_empty_graph() {
        let graph: MGraph<&str> = MGraph::new();
        
        assert_eq!(graph.vertex_count(), 0);
        assert_eq!(graph.first_neighbor(0), None);
    }

    #[test]
    fn test_disconnected_graph() {
        let mut graph = MGraph::new();
        
        let id0 = graph.add_node("A");
        let id1 = graph.add_node("B");
        let id2 = graph.add_node("C");
        
        graph.add_edge(id0, id1, 1);
        graph.add_edge(id1, id0, 1);
        
        let mut visited = Vec::new();
        graph.bfs_traverse(id0, |id| visited.push(id));
        
        assert_eq!(visited, vec![id0, id1]);
        assert!(!visited.contains(&id2));
    }
}