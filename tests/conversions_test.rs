mod common;

use redisgraph::{RedisString, result_set::{Node, Relation, Scalar}};
use serial_test::serial;
use maplit::hashmap;

use common::*;

#[test]
#[serial]
fn test_scalar() {
    with_graph(|graph| {
        let scalar: Scalar = graph.query("RETURN 42").unwrap();
        assert_eq!(scalar, Scalar::Integer(42));
    });
}

#[test]
#[serial]
fn test_nil() {
    #[allow(clippy::let_unit_value)]
    with_graph(|graph| {
        let _nil: () = graph.query("RETURN null").unwrap();
    });
}

#[test]
#[serial]
fn test_option() {
    with_graph(|graph| {
        let results: (Option<i64>, Option<i64>) = graph.query("RETURN 42, null").unwrap();
        assert_eq!(results.0, Some(42));
        assert_eq!(results.1, None);
    });
}

#[test]
#[serial]
fn test_bool() {
    with_graph(|graph| {
        let boolean: bool = graph.query("RETURN true").unwrap();
        assert_eq!(boolean, true);
    });
}

#[test]
#[serial]
fn test_int() {
    with_graph(|graph| {
        let integer: i64 = graph.query("RETURN 42").unwrap();
        assert_eq!(integer, 42);
    });
}

#[test]
#[serial]
fn test_float() {
    #[allow(clippy::float_cmp)]
    with_graph(|graph| {
        let float: f64 = graph.query("RETURN 12.3").unwrap();
        assert_eq!(float, 12.3);
    });
}

#[test]
#[serial]
fn test_redis_string() {
    with_graph(|graph| {
        let redis_string: RedisString = graph.query("RETURN 'Hello, world!'").unwrap();
        assert_eq!(redis_string, "Hello, world!".to_string().into());
    });
}

#[test]
#[serial]
fn test_string() {
    with_graph(|graph| {
        let string: String = graph.query("RETURN 'Hello again, world!'").unwrap();
        assert_eq!(string, "Hello again, world!".to_string());
    });
}

#[test]
#[serial]
fn test_node() {
    with_graph(|graph| {
        graph.mutate("CREATE (n:NodeLabel { prop: 42 })").unwrap();
        let node: Node = graph.query("MATCH (n) RETURN n").unwrap();
        assert_eq!(node, Node {
            labels: vec!["NodeLabel".to_string().into()],
            properties: hashmap! {
                "prop".to_string().into() => Scalar::Integer(42),
            },
        });
    });
}

#[test]
#[serial]
fn test_relation() {
    with_graph(|graph| {
        graph.mutate("CREATE (src)-[rel:RelationType { prop: 42 }]->(dst)").unwrap();
        let relation: Relation = graph.query("MATCH (src)-[rel]->(dst) RETURN rel").unwrap();
        assert_eq!(relation, Relation {
            type_name: "RelationType".to_string().into(),
            properties: hashmap! {
                "prop".to_string().into() => Scalar::Integer(42),
            },
        });
    });
}
