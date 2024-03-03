use crate::types::Unit;
use alloc::vec::Vec;
use hashbrown::HashMap;

type NodeId = u32;

pub struct Tree<Data = ()> {
    root: Node<Data>,
    /// the collection of all the ndoes in the tree
    // nodes: Vec<Node<Data>>,
    nodes: HashMap<NodeId, Node<Data>>,
    /// the finalized sizes and location for each node
    computed_sizes: HashMap<NodeId, (u32, u32)>
}

pub struct Node<Data> {
    // id: NodeId,
    data: NodeData<Data>,
    children: Vec<NodeId>,
}

pub struct NodeData<Data> {
    /// the node's associated data
    data: Option<Data>,
    /// how the node wishes to be laid out
    layout: Unit,
}
