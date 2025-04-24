pub mod dynamic_linked_list;
pub mod static_linked_list;

/// A trait defining the interface for all linked list implementations.
pub trait LinkedListTrait<T> {
    fn insert(&mut self, data: T);
    fn insert_at_index(&mut self, index: usize, data: T) -> Result<(), String>;
    fn delete_element(&mut self, data: T) -> bool;
    fn delete_at_index(&mut self, index: usize) -> Result<(), String>;
    fn update_element(&mut self, old_data: T, new_data: T) -> bool;
    fn update_element_at_index(&mut self, index: usize, data: T) -> Result<(), String>;
    fn find(&self, data: &T) -> bool;
    fn get(&self, index: usize) -> Option<&T>;
}
