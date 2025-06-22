mod arena;
mod client;

#[cfg(test)]
mod tests {
    use crate::arena::arena::ArenaTree;
    use crate::arena::obj::Object;

    #[test]
    fn initialize() {
        let t = ArenaTree::new(None);
        assert_eq!(t.current_position(), None);
        assert_eq!(t.get_current_objects_children(), None);
    }
    
    #[test]
    fn append_node() {
        let mut t = ArenaTree::new(None);
        t.add_node("node");
        let expected_position = Some(0);
        let actual_position = t.current_position();
        let expected_object = Some(Object::new("node", 0, None));
        assert_eq!(actual_position, expected_position);
        assert_eq!(t.get_current_objects_children(), None);
        assert_eq!(t.get_object(), expected_object);
    }
    #[test]
    fn append_nodes() {
    
        let mut t = ArenaTree::new(None);
        t.add_node("node");
        let expected_position = Some(0);
        let actual_position = t.current_position();
        let expected_object1 = Some(Object::new("node", 0, None));
        assert_eq!(actual_position, expected_position);
        assert_eq!(t.get_current_objects_children(), None);
        assert_eq!(t.get_object(), expected_object1);
        
        t.add_node("another_node");
        let expected_position = Some(0);
        let actual_position = t.current_position();
        let expected_object2 = Some(Object::new("another_node", 1, None));
        assert_eq!(actual_position, expected_position);
        assert_eq!(t.get_current_objects_children(), None);
        assert_eq!(t.get_object(), expected_object1);
        
    }
    
    #[test]
    fn append_child_node() {
    
        let mut t = ArenaTree::new(None);
        t.add_node("parent");
        let expected_position = Some(0);
        let actual_position = t.current_position();
        let expected_object1 = Some(Object::new("parent", 0, None));
        assert_eq!(actual_position, expected_position);
        assert_eq!(t.get_current_objects_children(), None);
        assert_eq!(t.get_object(), expected_object1);
        
        t.add_child_node("child", 0);
        let expected_parent = Some(Object::new_with_children("parent", 0, None, Some(vec![1])));
        assert_eq!(t.get_object(), expected_parent);
        
        let expected_position = Some(0);
        let actual_position = t.current_position();
        let child_object = Object::new("child", 1, Some(0));
        let expected_child_object = Some(child_object.clone());
        let expected_child_of_object_1 = Some(vec![child_object.clone()]);
        
        
        assert_eq!(actual_position, expected_position);
        assert_eq!(t.get_current_objects_children(), expected_child_of_object_1);
        t.next_position();
        assert_eq!(t.get_object(), expected_child_object);
        
    }
    
        #[test]
    fn append_children_nodes() {
            
            let mut t = ArenaTree::new(None);
            t.add_node("parent");
            let expected_position = Some(0);
            let actual_position = t.current_position();
            let expected_object1 = Some(Object::new("parent", 0, None));
            assert_eq!(actual_position, expected_position);
            assert_eq!(t.get_current_objects_children(), None);
            assert_eq!(t.get_object(), expected_object1);
            
            // add children dont move
            t.add_child_node("child1", 0);
            let expected_parent = Some(Object::new_with_children("parent", 0, None, Some(vec![1])));
            assert_eq!(t.get_object(), expected_parent);
            t.add_child_node("child2", 0);
            let expected_parent = Some(Object::new_with_children("parent", 0, None, Some(vec![1, 2])));
            assert_eq!(t.get_object(), expected_parent);
            
            // move to child 1
            t.next_position();
            let expected_position = Some(1);
            let actual_position = t.current_position();
            assert_eq!(actual_position, expected_position);
            let expected_child_object_1 = Some(Object::new("child1", 1, Some(0)));
            assert_eq!(t.get_object(), expected_child_object_1);
            
            // move to child 2
            t.next_position();
            let expected_position = Some(2);
            let actual_position = t.current_position();
            assert_eq!(actual_position, expected_position);
            let expected_child_object_2 = Some(Object::new("child2", 2, Some(0)));
            assert_eq!(t.get_object(), expected_child_object_2);
            
            // move to child 3
            t.next_position();
            let expected_position = Some(3);
            let actual_position = t.current_position();
            assert_eq!(actual_position, expected_position);
        }
    
    
}
