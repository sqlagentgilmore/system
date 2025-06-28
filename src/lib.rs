pub mod arena;
mod node;
mod value;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use compact_str::{CompactString, ToCompactString};
    use crate::arena::Arena;
    use crate::node::{Node, Value};

    #[test]
    fn test_new() {
        let mut tree = Arena::new();
        assert!(tree.len().eq(&0));
    }

    #[test]
    fn test_one_node() {
        let mut tree = Arena::new();
        assert!(tree.len().eq(&1));
        assert_eq!(tree.root_index(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut tree = Arena::new();
        tree.add_node("node", "database", 0);
        tree.add_node("child_node", "schema", 1);
        tree.add_node("another_node", "schema", 0);
        assert_eq!(tree.len(), 4);
        let parents = tree.get_owned_parent_nodes(2);
        let mut expected_root = Node::root();
        expected_root.children = Some(vec![1, 3]);
        let mut expected_child = Node::new("node", "database", 1, 0);
        expected_child.children = Some(vec![2]);
        let expected = vec![expected_child, expected_root];
        for i in 0..parents.len() {
            assert_eq!(parents[i].children, expected[i].children);
            assert_eq!(parents[i].value, expected[i].value);
            assert_eq!(parents[i].index, expected[i].index);
            assert_eq!(parents[i].parent, expected[i].parent);
        }
    }
}
