use ipe::{Error, Leaf, Node, NodeKind, Registry, Status, Tree};

struct Always;
impl Leaf<(), (), (), ()> for Always {
    const KIND: &'static str = "always";
    type Config = ();
    fn tick(_: &(), _: &(), _: &(), _: &mut (), _: &mut ()) -> Status {
        Status::Success
    }
}

fn leaf(id: &str) -> Node {
    Node {
        id: id.into(),
        kind: NodeKind::Leaf {
            kind: "always".into(),
            config: serde_json::Value::Null,
        },
    }
}

#[test]
fn reexecutes_from_root() {
    let tree = Tree {
        root: "root".into(),
        nodes: vec![
            Node {
                id: "root".into(),
                kind: NodeKind::Sequence {
                    children: vec!["a".into(), "b".into()],
                },
            },
            leaf("a"),
            leaf("b"),
        ],
    };

    let mut registry = Registry::<(), (), (), ()>::new();
    registry.register::<Always>().unwrap();

    let compiled = registry.compile(&tree).unwrap();

    assert_eq!(compiled.tick(&(), &(), &mut (), &mut ()), Status::Success);
    assert_eq!(compiled.tick(&(), &(), &mut (), &mut ()), Status::Success);
    assert_eq!(compiled.tick(&(), &(), &mut (), &mut ()), Status::Success);
}

#[test]
fn rejects_cycles() {
    let tree = Tree {
        root: "root".into(),
        nodes: vec![Node {
            id: "root".into(),
            kind: NodeKind::Inverter {
                child: "root".into(),
            },
        }],
    };
    let registry = Registry::<(), (), (), ()>::new();
    assert!(matches!(registry.compile(&tree), Err(Error::Cycle(_))));
}

#[test]
fn json_roundtrip() {
    let tree = Tree {
        root: "x".into(),
        nodes: vec![leaf("x")],
    };
    let copy: Tree = serde_json::from_str(&serde_json::to_string(&tree).unwrap()).unwrap();
    assert_eq!(copy.nodes.len(), 1);
}
