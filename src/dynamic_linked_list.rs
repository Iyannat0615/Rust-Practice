use std::fmt::Debug;

use crate::LinkedListTrait;

/// `Node` represents a single element in the dynamic linked list.
/// 
/// Each node stores data of type `T` and a pointer to the next node.
#[derive(Debug)]
struct Node<T> {
    /// The data stored in the node.
    data: T,
    /// A pointer to the next node in the list.
    next: Option<Box<Node<T>>>,
}

/// `DynamicLinkedList` is a singly linked list that uses dynamic memory allocation.
///
/// It supports common linked list operations such as insertion, deletion, update,
/// retrieval, and search.
#[derive(Debug)]
pub struct DynamicLinkedList<T> {
    /// A pointer to the head (first element) of the linked list.
    head: Option<Box<Node<T>>>,
}

impl<T> DynamicLinkedList<T> {
    /// Creates a new, empty `DynamicLinkedList`.
    ///
    /// # Returns
    /// - A new empty `DynamicLinkedList` instance.
    pub fn new() -> Self {
        DynamicLinkedList { head: None }
    }
}

impl<T: PartialEq + Clone + Debug> LinkedListTrait<T> for DynamicLinkedList<T> {
    /// Inserts an element at the end (tail) of the list.
    ///
    /// # Parameters
    /// - `data`: The value to insert.
    fn insert(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });

        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            }
            Some(mut current) => {
                while current.next.is_some() {
                    let next = current.next.as_mut().unwrap();
                    current = next;
                }
                current.next = Some(new_node);
            }
        }
    }

    /// Inserts an element at a specific index in the list.
    ///
    /// # Parameters
    /// - `index`: The position to insert at (0-based).
    /// - `data`: The value to insert.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err("Index out of bounds")` if the index is invalid.
    fn insert_at_index(&mut self, index: usize, data: T) -> Result<(), String> {
        if index == 0 {
            let new_node = Box::new(Node {
                data,
                next: self.head.take(),
            });
            self.head = Some(new_node);
            return Ok(());
        }

        let mut current = &mut self.head;
        for _ in 0..(index - 1) {
            match current {
                Some(node) => {
                    current = &mut node.next;
                }
                None => {
                    return Err("Index out of bounds".to_string());
                }
            }
        }

        match current {
            Some(node) => {
                let new_node = Box::new(Node {
                    data,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
                Ok(())
            }
            None => Err("Index out of bounds".to_string()),
        }
    }

    /// Deletes the first occurrence of the given value from the list.
    ///
    /// # Parameters
    /// - `data`: The value to delete.
    ///
    /// # Returns
    /// - `true` if the value was found and removed.
    /// - `false` if the value was not found.
    fn delete_element(&mut self, data: T) -> bool {
        if self.head.is_none() {
            return false;
        }

        if self.head.as_ref().unwrap().data == data {
            self.head = self.head.take().unwrap().next;
            return true;
        }

        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.next.is_some() && node.next.as_ref().unwrap().data == data {
                node.next = node.next.take().unwrap().next;
                return true;
            }
            current = &mut node.next;
        }

        false
    }

    /// Deletes the element at the specified index.
    ///
    /// # Parameters
    /// - `index`: The index of the element to delete.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err("Index out of bounds")` if the index is invalid.
    fn delete_at_index(&mut self, index: usize) -> Result<(), String> {
        if index == 0 {
            if self.head.is_none() {
                return Err("Index out of bounds".to_string());
            }
            self.head = self.head.take().unwrap().next;
            return Ok(());
        }

        let mut current = &mut self.head;
        for _ in 0..(index - 1) {
            match current {
                Some(node) => {
                    current = &mut node.next;
                }
                None => {
                    return Err("Index out of bounds".to_string());
                }
            }
        }

        match current {
            Some(node) => {
                if node.next.is_none() {
                    return Err("Index out of bounds".to_string());
                }
                node.next = node.next.take().unwrap().next;
                Ok(())
            }
            None => Err("Index out of bounds".to_string()),
        }
    }

    /// Updates the first node that matches `old_data` with `new_data`.
    ///
    /// # Parameters
    /// - `old_data`: The value to replace.
    /// - `new_data`: The new value.
    ///
    /// # Returns
    /// - `true` if an update occurred.
    /// - `false` if the old value was not found.
    fn update_element(&mut self, old_data: T, new_data: T) -> bool {
        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.data == old_data {
                node.data = new_data;
                return true;
            }
            current = &mut node.next;
        }
        false
    }

    /// Updates the data of the node at the specified index.
    ///
    /// # Parameters
    /// - `index`: The index of the node to update.
    /// - `data`: The new value to set.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err("Index out of bounds")` if the index is invalid.
    fn update_element_at_index(&mut self, index: usize, data: T) -> Result<(), String> {
        let mut current = &mut self.head;
        for _ in 0..index {
            match current {
                Some(node) => {
                    current = &mut node.next;
                }
                None => {
                    return Err("Index out of bounds".to_string());
                }
            }
        }

        match current {
            Some(node) => {
                node.data = data;
                Ok(())
            }
            None => Err("Index out of bounds".to_string()),
        }
    }

    /// Checks whether a given value exists in the list.
    ///
    /// # Parameters
    /// - `data`: A reference to the value to search for.
    ///
    /// # Returns
    /// - `true` if the value exists in the list.
    /// - `false` otherwise.
    fn find(&self, data: &T) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if &node.data == data {
                return true;
            }
            current = &node.next;
        }
        false
    }

    /// Returns a reference to the data at the specified index.
    ///
    /// # Parameters
    /// - `index`: The index of the element to retrieve.
    ///
    /// # Returns
    /// - `Some(&T)` if the index is valid.
    /// - `None` otherwise.
    fn get(&self, index: usize) -> Option<&T> {
        let mut current = &self.head;
        for _ in 0..index {
            match current {
                Some(node) => {
                    current = &node.next;
                }
                None => {
                    return None;
                }
            }
        }

        match current {
            Some(node) => Some(&node.data),
            None => None,
        }
    }
}
