pub(crate) mod compiled;
pub(crate) mod error;
pub(crate) mod registry;
pub(crate) mod status;

pub mod leaves;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

pub use crate::{compiled::CompiledTree, error::Error, registry::Registry, status::Status};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub root: String,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    #[serde(flatten)]
    pub kind: NodeKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeKind {
    Sequence {
        children: Vec<String>,
    },
    Selector {
        children: Vec<String>,
    },
    Inverter {
        child: String,
    },
    Leaf {
        kind: String,
        #[serde(default)]
        config: Value,
    },
}

pub trait Leaf<World, State, Action, Board>: Send + Sync + 'static {
    const KIND: &'static str;
    type Config: DeserializeOwned + Send + Sync + 'static;
    fn tick(
        config: &Self::Config,
        world: &World,
        state: &State,
        action: &mut Action,
        board: &mut Board,
    ) -> Status;
}
