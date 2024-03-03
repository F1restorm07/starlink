use crate::types::{ Unit, Orientation };
use alloc::vec::Vec;
use hashbrown::HashMap;

type NodeId = u32;

/// a layout tree computed in a `top-down` fashion
#[derive(Debug)]
pub struct Tree<Data: Clone> {
    root_node: Node<Data>,
    /// the collection of all the ndoes in the tree
    nodes: HashMap<NodeId, Node<Data>>,
    /// the finalized sizes and location for each node
    computed_sizes: HashMap<NodeId, (u32, u32)>
}

/// a layout node with optional assoicated data and children
#[derive(Debug, Clone)]
pub struct Node<Data: Clone> {
    data: NodeData<Data>,
    children: Vec<NodeId>,
}

#[derive(Debug, Clone)]
struct NodeData<Data> {
    /// the node's associated data
    data: Option<Data>,
    /// how the node wishes to be laid out
    strat: Unit,
    /// how the node's children will be laid out
    orientation: Orientation,
}

impl<Data: Clone> Default for Tree<Data> {
    fn default() -> Self { Self::new(Node::new(Unit::Fill, Orientation::Horizontal))}
}

impl<Data: Clone> Tree<Data> {
    pub fn new(root_node: Node<Data>) -> Self {
        let mut nodes = HashMap::with_capacity(16);
        nodes.insert(nodes.len() as u32 + 1, root_node.clone());
        Self {
            root_node,
            nodes,
            computed_sizes: HashMap::with_capacity(16),
        }
    }
    pub fn add_node(&mut self, node: Node<Data>, parent: Option<NodeId>) {
        let new_node_id = self.nodes.len() as u32 + 1;
        self.nodes.insert(new_node_id, node);

        if let Some(id) = parent {
            self.nodes.get_mut(&id).unwrap().add_child(new_node_id);
        }
    }
    pub fn compute_layout(&mut self, node: NodeId, space: (u32, u32)) {

    }
}

impl<Data: Clone> Node<Data> {
    pub fn new(strat: Unit, orientation: Orientation) -> Self {
        Self { data: NodeData { data: None, strat, orientation }, children: Vec::new() }
    }
    pub fn add_data(&mut self, data: Data) { self.data.data = Some(data); }
    pub fn add_child(&mut self, node: NodeId) { self.children.push(node); }
}
