use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use rustc_hash::FxHashMap;

use super::visitor::SymbolRef;

pub struct SymbolGraph {
  pub(crate) graph: StableDiGraph<SymbolRef, ()>,
  symbol_to_index: FxHashMap<SymbolRef, NodeIndex>,
}

impl SymbolGraph {
  pub fn new() -> Self {
    Self {
      graph: StableDiGraph::new(),
      symbol_to_index: FxHashMap::default(),
    }
  }

  pub fn add_node(&mut self, symbol: &SymbolRef) -> NodeIndex {
    if let Some(index) = self.symbol_to_index.get(symbol) {
      *index
    } else {
      let index = self.graph.add_node(symbol.clone());
      self.symbol_to_index.insert(symbol.clone(), index);
      index
    }
  }

  pub fn has_node(&mut self, symbol: &SymbolRef) -> bool {
    self.symbol_to_index.contains_key(symbol)
  }

  pub fn add_edge(&mut self, from: &SymbolRef, to: &SymbolRef) {
    let from_index = self.add_node(from);
    let to_index = self.add_node(to);
    if !self.graph.contains_edge(from_index, to_index) {
      self.graph.add_edge(from_index, to_index, ());
    }
  }
}
