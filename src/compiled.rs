use std::{any::Any, marker::PhantomData, sync::Arc};

use serde_json::Value;

use crate::{status::Status, Leaf};

pub trait ErasedLeaf<World, State, Action, Board>: Send + Sync {
    fn decode(&self, config: Value) -> Result<Arc<dyn Any + Send + Sync>, serde_json::Error>;
    fn tick(
        &self,
        config: &(dyn Any + Send + Sync),
        world: &World,
        state: &State,
        action: &mut Action,
        board: &mut Board,
    ) -> Status;
}

pub struct Adapter<T>(pub PhantomData<T>);
impl<W, S, A, B, T: Leaf<W, S, A, B>> ErasedLeaf<W, S, A, B> for Adapter<T> {
    fn decode(&self, config: Value) -> Result<Arc<dyn Any + Send + Sync>, serde_json::Error> {
        Ok(Arc::new(serde_json::from_value::<T::Config>(config)?))
    }

    fn tick(
        &self,
        config: &(dyn Any + Send + Sync),
        world: &W,
        state: &S,
        action: &mut A,
        board: &mut B,
    ) -> Status {
        let config = config
            .downcast_ref::<T::Config>()
            .expect("compiler guaranteed config type");

        T::tick(config, world, state, action, board)
    }
}

pub enum CompiledNode<W, S, A, B> {
    // composite nodes
    Parallel(Vec<usize>),
    Sequence(Vec<usize>),
    Selector(Vec<usize>),
    // decorator nodes
    Inverter(usize),
    Succeeder(usize),
    Failer(usize),
    // leaves
    Leaf {
        handler: Arc<dyn ErasedLeaf<W, S, A, B>>,
        config: Arc<dyn Any + Send + Sync>,
    },
}

pub struct CompiledTree<W, S, A, B> {
    pub root: usize,
    pub nodes: Vec<CompiledNode<W, S, A, B>>,
}

impl<W, S, A, B> CompiledTree<W, S, A, B> {
    pub fn tick(&self, world: &W, state: &S, action: &mut A, board: &mut B) -> Status {
        self.eval(self.root, world, state, action, board)
    }

    fn eval(&self, i: usize, world: &W, state: &S, action: &mut A, board: &mut B) -> Status {
        match &self.nodes[i] {
            CompiledNode::Parallel(children) => {
                for &child in children {
                    let _ = self.eval(child, world, state, action, board);
                }
                Status::Success
            }
            CompiledNode::Sequence(children) => {
                for &child in children {
                    match self.eval(child, world, state, action, board) {
                        Status::Success => {}
                        other => return other,
                    }
                }
                Status::Success
            }
            CompiledNode::Selector(children) => {
                for &child in children {
                    match self.eval(child, world, state, action, board) {
                        Status::Failure => {}
                        other => return other,
                    }
                }
                Status::Failure
            }
            CompiledNode::Inverter(child) => match self.eval(*child, world, state, action, board) {
                Status::Idle => Status::Idle,
                Status::Running => Status::Running,
                Status::Success => Status::Failure,
                Status::Failure => Status::Success,
                Status::Error => Status::Error,
            },
            CompiledNode::Succeeder(child) => {
                let _ = self.eval(*child, world, state, action, board);
                Status::Success
            }
            CompiledNode::Failer(child) => {
                let _ = self.eval(*child, world, state, action, board);
                Status::Failure
            }
            CompiledNode::Leaf { handler, config } => {
                handler.tick(config.as_ref(), world, state, action, board)
            }
        }
    }
}
