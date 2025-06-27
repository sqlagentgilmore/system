pub mod arena;
mod node;
mod value;

#[cfg(test)]
mod tests {
    use crate::arena::Arena;

    #[test]
    fn test_new() {
        let mut tree = Arena::new();
        assert!(tree.len().eq(&0));
    }

    #[test]
    fn test_one_node() {
        let mut tree = Arena::new();
        tree.add_root_node("node");
        assert!(tree.len().eq(&1));
        assert_eq!(tree.index(), Some(0));
    }

    #[test]
    fn test_one_node_and_child() {
        let mut tree = Arena::new();
        tree.add_root_node("node");
        tree.add_child_node("tiny_node");
        assert!(tree.len().eq(&2));
        assert_eq!(tree.index(), Some(0));
    }
    #[test]
    fn test_multi_level() {
        let mut tree = Arena::new();
        tree.add_root_node("node");
        tree.add_child_node("tiny_node");
        tree.advance();
        tree.add_child_node("tiniest node");
        tree.advance();
        let expected_lineage = vec![1, 0];
        let actual_lineage = tree.lineage();
        assert_eq!(actual_lineage, expected_lineage);
        tree.move_to_parent();
        tree.add_child_node("other tiniest node");
        let subtree = tree.subtree();
        println!("{tree}");
        println!("{subtree}");
        assert_eq!(subtree.len(), 3);

    }
}
