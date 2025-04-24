// dynamic_linked_list_test.rs
// This file contains unit tests for the DynamicLinkedList implementation.
// It tests various list operations such as insertion, deletion, updating, and getting elements.

#[cfg(test)]
mod dynamic_linked_list_tests {
    use linked_list_impls::dynamic_linked_list::DynamicLinkedList;
    use linked_list_impls::LinkedListTrait;

    // Mock data type for testing. This will be used to test the linked list functionality.
    #[derive(Debug, PartialEq, Eq, Clone)]
    struct TestData {
        value: i32,
    }

    /// Test inserting a new element into the dynamic linked list.
    #[test]
    fn test_insert() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        list.insert(TestData { value: 1 });
        assert_eq!(list.get(0).unwrap().value, 1); // Ensure the first element is 1.
    }

    /// Test inserting an element at a specific index in the dynamic linked list.
    #[test]
    fn test_insert_at_index() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        list.insert(TestData { value: 1 });
        list.insert_at_index(0, TestData { value: 2 }).unwrap();
        assert_eq!(list.get(0).unwrap().value, 2); // Ensure first element is 2.
        assert_eq!(list.get(1).unwrap().value, 1); // Ensure second element is 1.
    }

    /// Test that attempting to insert at an out-of-bounds index returns an error.
    #[test]
    fn test_insert_at_index_out_of_bounds() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        let result = list.insert_at_index(1, TestData { value: 2 });
        assert!(result.is_err()); // List should not allow out-of-bounds insertions.
    }

    /// Test deleting an element from the list.
    #[test]
    fn test_delete_element() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        list.insert(TestData { value: 1 });
        list.insert(TestData { value: 2 });
        assert_eq!(list.delete_element(TestData { value: 1 }), true); // Ensure deletion is successful.
        assert_eq!(list.find(&TestData { value: 1 }), false); // Ensure element is removed.
        assert_eq!(list.get(0).unwrap().value, 2); // Ensure list still contains remaining elements.
    }

    /// Test trying to delete an element that doesn't exist in the list.
    #[test]
    fn test_delete_element_not_found() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        list.insert(TestData { value: 1 });
        assert_eq!(list.delete_element(TestData { value: 2 }), false); // Ensure deletion fails for non-existent element.
    }

    /// Test deleting an element at a specific index.
    #[test]
    fn test_delete_at_index() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        list.insert(TestData { value: 1 });
        list.insert(TestData { value: 2 });
        list.delete_at_index(0).unwrap();
        assert_eq!(list.find(&TestData { value: 1 }), false); // Ensure the first element is removed.
        assert_eq!(list.get(0).unwrap().value, 2); // Ensure the second element is now the first.
    }

    /// Test attempting to delete an element at an invalid index.
    #[test]
    fn test_delete_at_index_out_of_bounds() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        let result = list.delete_at_index(0);
        assert!(result.is_err()); // Ensure deletion fails for invalid index.
    }

    /// Test updating an existing element in the list.
    #[test]
    fn test_update_element() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        list.insert(TestData { value: 1 });
        list.update_element(TestData { value: 1 }, TestData { value: 2 });
        assert_eq!(list.get(0).unwrap().value, 2); // Ensure the element is updated to 2.
    }

    /// Test trying to update a non-existent element in the list.
    #[test]
    fn test_update_element_not_found() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        list.insert(TestData { value: 1 });
        assert_eq!(list.update_element(TestData { value: 2 }, TestData { value: 3 }), false); // Ensure update fails for non-existent element.
    }

    /// Test updating an element at a specific index.
    #[test]
    fn test_update_element_at_index() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        list.insert(TestData { value: 1 });
        list.update_element_at_index(0, TestData { value: 2 }).unwrap();
        assert_eq!(list.get(0).unwrap().value, 2); // Ensure the element at index 0 is updated to 2.
    }

    /// Test attempting to update an element at an out-of-bounds index.
    #[test]
    fn test_update_element_at_index_out_of_bounds() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        let result = list.update_element_at_index(0, TestData { value: 2 });
        assert!(result.is_err()); // Ensure update fails for out-of-bounds index.
    }

    /// Test finding an element in the list.
    #[test]
    fn test_find() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        list.insert(TestData { value: 1 });
        assert_eq!(list.find(&TestData { value: 1 }), true); // Ensure element is found.
        assert_eq!(list.find(&TestData { value: 2 }), false); // Ensure element is not found.
    }

    /// Test getting an element at a specific index.
    #[test]
    fn test_get() {
        let mut list: DynamicLinkedList<TestData> = DynamicLinkedList::new();
        list.insert(TestData { value: 1 });
        assert_eq!(list.get(0).unwrap().value, 1); // Ensure correct value is retrieved.
        assert_eq!(list.get(1), None); // Ensure out-of-bounds index returns None.
    }
}
