use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    sync::Arc,
};

use crate::{
    compiled::{Adapter, CompiledNode, CompiledTree, ErasedLeaf},
    error::Error,
    Leaf, NodeKind, Tree,
};

pub struct Registry<W, S, A, B> {
    handlers: HashMap<String, Arc<dyn ErasedLeaf<W, S, A, B>>>,
}

impl<W: 'static, S: 'static, A: 'static, B: 'static> Default for Registry<W, S, A, B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<W: 'static, S: 'static, A: 'static, B: 'static> Registry<W, S, A, B> {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<T: Leaf<W, S, A, B>>(&mut self) -> Result<(), Error> {
        // check duplication
        if self.handlers.contains_key(T::KIND) {
            return Err(Error::DuplicateLeaf(T::KIND.into()));
        }
        self.handlers
            .insert(T::KIND.into(), Arc::new(Adapter::<T>(PhantomData)));
        Ok(())
    }

    pub fn compile(&self, tree: &Tree) -> Result<CompiledTree<W, S, A, B>, Error> {
        let mut ids = HashMap::new();
        for (i, node) in tree.nodes.iter().enumerate() {
            if ids.insert(node.id.as_str(), i).is_some() {
                return Err(Error::DuplicateNode(node.id.clone()));
            }
        }
        let root = *ids
            .get(tree.root.as_str())
            .ok_or_else(|| Error::MissingNode(tree.root.clone()))?;

        let mut nodes = Vec::with_capacity(tree.nodes.len());
        for node in &tree.nodes {
            let get = |id: &str| {
                ids.get(id)
                    .copied()
                    .ok_or_else(|| Error::MissingNode(id.to_owned()))
            };
            let compiled = match &node.kind {
                NodeKind::Sequence { children } => CompiledNode::Sequence(
                    children
                        .iter()
                        .map(|id| get(id))
                        .collect::<Result<_, _>>()?,
                ),
                NodeKind::Selector { children } => CompiledNode::Selector(
                    children
                        .iter()
                        .map(|id| get(id))
                        .collect::<Result<_, _>>()?,
                ),
                NodeKind::Inverter { child } => CompiledNode::Inverter(get(child)?),
                NodeKind::Leaf { kind, config } => {
                    let handler = self
                        .handlers
                        .get(kind)
                        .ok_or_else(|| Error::UnknownLeaf(kind.clone()))?
                        .clone();
                    let decoded =
                        handler
                            .decode(config.clone())
                            .map_err(|e| Error::InvalidConfig {
                                node: node.id.clone(),
                                reason: e.to_string(),
                            })?;
                    CompiledNode::Leaf {
                        handler,
                        config: decoded,
                    }
                }
            };
            nodes.push(compiled);
        }
        // Validate all nodes, including unreachable ones, so malformed definitions are rejected.
        let mut visited = HashSet::new();
        let mut active = HashSet::new();
        fn visit<W, S, A, B>(
            i: usize,
            nodes: &[CompiledNode<W, S, A, B>],
            visited: &mut HashSet<usize>,
            active: &mut HashSet<usize>,
            tree: &Tree,
        ) -> Result<(), Error> {
            if active.contains(&i) {
                return Err(Error::Cycle(tree.nodes[i].id.clone()));
            }
            if visited.contains(&i) {
                return Ok(());
            }
            active.insert(i);
            match &nodes[i] {
                // multiple child nodes (composite)
                CompiledNode::Parallel(children)
                | CompiledNode::Sequence(children)
                | CompiledNode::Selector(children) => {
                    for &child in children {
                        visit(child, nodes, visited, active, tree)?;
                    }
                }
                // single child node (decorator)
                CompiledNode::Inverter(child)
                | CompiledNode::Succeeder(child)
                | CompiledNode::Failer(child) => visit(*child, nodes, visited, active, tree)?,
                // no child node (leaf)
                CompiledNode::Leaf { .. } => {}
            }
            active.remove(&i);
            visited.insert(i);
            Ok(())
        }
        for i in 0..nodes.len() {
            visit(i, &nodes, &mut visited, &mut active, tree)?;
        }
        Ok(CompiledTree { root, nodes })
    }
}
