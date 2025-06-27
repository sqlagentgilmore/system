pub mod arena;
mod node;
mod value;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::arena::Arena;
    use crate::node::Value;

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
    
    #[test]
    fn test_from_two_level_hashmap() {
        // Create a 2-level structure:
        // root
        // ├── child1
        // └── child2
        let mut map: HashMap<Value, HashMap<Value, Value>> = HashMap::new();
        let mut children = HashMap::new();
        children.insert("child1".into(), "leaf1".into());
        children.insert("child2".into(), "leaf2".into());
        map.insert("root".into(), children);

        let arena = Arena::from(map);
        println!("{}",arena);
        // Verify structure
        assert_eq!(arena.len(), 3); // root + 2 children
        
        // Check root
        let root = arena.node_from_index(0).unwrap();
        assert_eq!(root.value(), "root".into());
        assert_eq!(root.parent(), None);
        assert_eq!(root.children().map(|c| c.len()), Some(2));

        // Check children
        let child1 = arena.node_from_index(1).unwrap();
        assert!(child1.value().eq( &"child1".into()) || child1.value().eq( &"child2".into()));
        assert_eq!(child1.parent(), Some(0));

        let child2 = arena.node_from_index(2).unwrap();
        assert!(child1.value().eq( &"child1".into()) || child1.value().eq( &"child2".into()));
        assert_eq!(child2.parent(), Some(0));
    }

    #[test]
    fn test_from_three_level_hashmap() {
        // Create a 3-level structure:
        // root
        // ├── branch1
        // │   ├── leaf1_1
        // │   └── leaf1_2
        // └── branch2
        //     └── leaf2_1
        let mut map: HashMap<Value, HashMap<Value, HashMap<Value, Value>>> = HashMap::new();
        
        let mut level1 = HashMap::new();
        let mut branch1_children = HashMap::new();
        branch1_children.insert("leaf1_1".into(), "data1".into());
        branch1_children.insert("leaf1_2".into(), "data2".into());
        
        let mut branch2_children = HashMap::new();
        branch2_children.insert("leaf2_1".into(), "data3".into());
        
        level1.insert("branch1".into(), branch1_children);
        level1.insert("branch2".into(), branch2_children);
        
        map.insert("root".into(), level1);

        let arena = Arena::from(map);

        // Verify structure
        assert_eq!(arena.len(), 6); // 1 root + 2 branches + 3 leaves
        
        // Check root
        let root = arena.node_from_index(0).unwrap();
        assert_eq!(root.value(), "root".into());
        assert_eq!(root.parent(), None);
        assert_eq!(root.children().map(|c| c.len()), Some(2));

        // Find and check branch1
        let branch1 = arena.node_by_value_from_index("branch1", 0).unwrap();
        assert_eq!(branch1.parent(), Some(0));
        assert_eq!(branch1.children().map(|c| c.len()), Some(2));

        // Find and check branch2
        let branch2 = arena.node_by_value_from_index("branch2", 0).unwrap();
        assert_eq!(branch2.parent(), Some(0));
        assert_eq!(branch2.children().map(|c| c.len()), Some(1));

        // Check leaf nodes have correct parents
        let leaf1_1 = arena.node_by_value_from_index("leaf1_1", 0).unwrap();
        assert!(leaf1_1.parent().is_some());
        
        let leaf2_1 = arena.node_by_value_from_index("leaf2_1", 0).unwrap();
        assert!(leaf2_1.parent().is_some());
    }

    #[test]
    fn test_from_multiple_roots_two_level() {
        // Test with multiple top-level entries
        let mut map: HashMap<Value, HashMap<Value, Value>> = HashMap::new();
        
        let mut children1 = HashMap::new();
        children1.insert("child1_1".into(), "data1".into());
        map.insert("root1".into(), children1);
        
        let mut children2 = HashMap::new();
        children2.insert("child2_1".into(), "data2".into());
        map.insert("root2".into(), children2);

        let arena = Arena::from(map);

        // Should create a tree where first entry is root and second is its child
        assert!(arena.len() >= 2);
        
        // Verify we have a connected tree, not a forest
        let actual_root = arena.node_from_index(0).unwrap();
        assert_eq!(actual_root.parent(), None);
    }

    #[test]
    fn test_empty_maps() {
        // Test empty 2-level map
        let empty_2level: HashMap<Value, HashMap<Value, Value>> = HashMap::new();
        let arena1 = Arena::from(empty_2level);
        assert!(arena1.is_empty());

        // Test empty 3-level map
        let empty_3level: HashMap<Value, HashMap<Value, HashMap<Value, Value>>> = HashMap::new();
        let arena2 = Arena::from(empty_3level);
        assert!(arena2.is_empty());
    }

    #[test]
    fn test_complex_three_level() {
        // Create a more complex structure to test index mapping
        let mut map: HashMap<Value, HashMap<Value, HashMap<Value, Value>>> = HashMap::new();
        
        let mut departments = HashMap::new();
        
        // Engineering department
        let mut eng_teams = HashMap::new();
        eng_teams.insert("frontend".into(), "Alice".into());
        eng_teams.insert("backend".into(), "Bob".into());
        eng_teams.insert("devops".into(), "Charlie".into());
        
        // Sales department
        let mut sales_teams = HashMap::new();
        sales_teams.insert("north".into(), "David".into());
        sales_teams.insert("south".into(), "Eve".into());
        
        departments.insert("engineering".into(), eng_teams);
        departments.insert("sales".into(), sales_teams);
        
        map.insert("company".into(), departments);

        let arena = Arena::from(map);

        // Verify we can traverse the tree
        assert_eq!(arena.node_from_index(0).unwrap().value(), "company".into());
        
        // Check that all nodes are connected
        let engineering = arena.node_by_value_from_index("engineering", 0).unwrap();
        assert_eq!(engineering.parent(), Some(0));
        assert!(engineering.children().is_some());
        
        // Verify leaf nodes
        let alice = arena.node_by_value_from_index("frontend", 0).unwrap();
        assert!(alice.parent().is_some());
        assert_eq!(alice.children(), None);
    }

    #[test]
    fn test_subtree_extraction() {
        // Build a tree and test subtree extraction
        let mut map: HashMap<Value, HashMap<Value, HashMap<Value, Value>>> = HashMap::new();
        let mut level1 = HashMap::new();
        let mut level2 = HashMap::new();
        level2.insert("grandchild".into(), "data".into());
        level1.insert("child".into(), level2);
        map.insert("root".into(), level1);

        let arena = Arena::from(map);
        
        // Find the child node
        let child_node = arena.node_by_value_from_index("child", 0).unwrap();
        let child_index = child_node.index();
        
        // Extract subtree starting from child
        let subtree = arena.subtree_from_index(child_index);
        
        // Verify subtree structure
        assert_eq!(subtree.len(), 2); // child + grandchild
        assert_eq!(subtree.node_from_index(0).unwrap().value(), "child".into());
        assert_eq!(subtree.node_from_index(0).unwrap().parent(), None); // Root of subtree
        assert_eq!(subtree.node_from_index(1).unwrap().value(), "grandchild".into());
        assert_eq!(subtree.node_from_index(1).unwrap().parent(), Some(0));
    }
}
