pub mod centre;
pub mod root;
mod isomorphic;
mod encode;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::marker::PhantomData;

trait GraphState {}

pub struct Unrooted;
pub struct Rooted;

impl GraphState for Unrooted {}
impl GraphState for Rooted {}

#[derive(Debug)]
pub struct Graph<T: Eq + Hash + Copy, S: GraphState> {
    pub nodes: HashMap<T, Rc<Node<T>>>,
    root: Option<Rc<Node<T>>>,
    _state: PhantomData<S>
}

impl<T: Eq + Hash + Copy> Graph<T, Unrooted> {
    pub fn with_capacity(capacity: usize) -> Self {
        Graph {
            nodes: HashMap::with_capacity(capacity),
            root: None,
            _state: PhantomData,
        }
    }

    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            root: None,
            _state: PhantomData,
        }
    }

    pub fn from_edge_list(edge_ls: Vec<(T, T)>, undirected: bool) -> Self {
        edge_ls.into_iter().fold(
            Graph::new(),
            |mut graph, (from, to)| {
                graph.add_edge(from, to, undirected);
                graph
            }
        )
    }

    pub fn from_adj_list(adj_ls: HashMap<T, HashSet<T>>, undirected: bool) -> Self {
        adj_ls.into_iter().fold(
            Graph::new(),
            |mut graph, (from, neighbours)| {
                for to in neighbours {
                    graph.add_edge(from, to, undirected);
                }
                graph
            }
        )
    }
}

impl<T: Eq + Hash + Copy, S: GraphState> Graph<T, S> {
    fn root_at(self, root: T) -> Graph<T, Rooted> {
        Graph {
            root: self.nodes.get(&root).cloned(),
            nodes: self.nodes,
            _state: PhantomData,
        }
    }
    
    pub fn add_edge(&mut self, from: T, to: T, undirected: bool) {
        let from = self.nodes
            .entry(from)
            .or_insert_with(|| Rc::new(Node::new(from)))
            .clone();
        let to = self.nodes
            .entry(to)
            .or_insert_with(|| Rc::new(Node::new(to)))
            .clone();

        if undirected {
            from.edges.borrow_mut().push(Edge::new(to.clone()));
            to.edges.borrow_mut().push(Edge::new(from));
        } else {
            from.edges.borrow_mut().push(Edge::new(to));
        }
    }
}

impl<T: Eq + Hash + Copy> Graph<T, Rooted> {
    pub fn root(&self) -> Rc<Node<T>> {
        self.root.clone().expect("infallible")
    }
}

#[derive(Debug)]
pub struct Node<T: Eq + Hash + Copy> {
    pub id: T,
    pub edges: RefCell<Vec<Edge<T>>>,
}

impl<T: Eq + Hash + Copy> Node<T> {
    pub fn new(id: T) -> Self {
        Node {
            id,
            edges: RefCell::new(Vec::new()),
        }
    }
    
    pub fn neighbours(&self) -> Vec<Rc<Node<T>>> {
        let mut edges = self.edges.borrow_mut();
        
        edges.retain(|edge| edge.to.strong_count() > 0);
        
        edges.iter()
            .map(|edge| edge.to.upgrade())
            .collect::<Option<_>>()
            .expect("infallible")
    }
}

#[derive(Debug)]
pub struct Edge<T: Eq + Hash + Copy> {
    pub to: Weak<Node<T>>
}

impl<T: Eq + Hash + Copy> Edge<T> {
    pub fn new(to: Rc<Node<T>>) -> Self {
        Edge { to: Rc::downgrade(&to) }
    }
}