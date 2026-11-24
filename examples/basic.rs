use ipe::{Leaf, Registry, Status, Tree};
use serde::Deserialize;

struct World {}

struct State {
    pub has_target: bool,
    pub distance: f32,
}

struct Action {
    pub commands: Vec<String>,
}

struct Board {}

struct HasTarget;
impl Leaf<World, State, Action, Board> for HasTarget {
    const KIND: &'static str = "basic.has_target";

    type Config = ();

    fn tick(_: &(), _: &World, state: &State, _: &mut Action, _: &mut Board) -> Status {
        if state.has_target {
            Status::Success
        } else {
            Status::Failure
        }
    }
}

#[derive(Deserialize)]
struct MoveConfig {
    speed: f32,
}
struct Move;
impl Leaf<World, State, Action, Board> for Move {
    const KIND: &'static str = "basic.move";

    type Config = MoveConfig;

    fn tick(
        config: &MoveConfig,
        _: &World,
        state: &State,
        action: &mut Action,
        _: &mut Board,
    ) -> Status {
        if state.distance <= 0.0 {
            Status::Success
        } else {
            action.commands.push(format!("move at {}", config.speed));
            Status::Running
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
      "root":"root",
      "nodes":[
        {"id":"root","type":"sequence","children":["check","move"]},
        {"id":"check","type":"leaf","kind":"basic.has_target","config":null},
        {"id":"move","type":"leaf","kind":"basic.move","config":{"speed":12.5}}
      ]
    }"#;

    let tree: Tree = serde_json::from_str(json)?;

    let mut registry = Registry::<World, State, Action, Board>::new();
    registry.register::<HasTarget>()?;
    registry.register::<Move>()?;

    let compiled = registry.compile(&tree)?;

    let world = World {};
    let mut state = State {
        has_target: true,
        distance: 100.0,
    };
    let mut action = Action {
        commands: Vec::new(),
    };
    let mut board = Board {};

    assert_eq!(
        compiled.tick(&world, &state, &mut action, &mut board),
        Status::Running
    );

    state.distance = 0.0;
    assert_eq!(
        compiled.tick(&world, &state, &mut action, &mut board),
        Status::Success
    );

    println!("commands: {:?}", action.commands);
    Ok(())
}
