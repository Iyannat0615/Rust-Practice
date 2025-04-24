// src/static_linked_list.rs

use std::fmt::Debug;

use crate::LinkedListTrait;

/// Node represents a single element in the static linked list.
#[derive(Debug, Clone)]
struct Node<T> {
    /// The data stored in the node.
    data: T,
    /// The index of the next node in the array.
    next: Option<usize>, // Index of the next node in the array
}

/// StaticLinkedList is a linked list implementation using a fixed-size array for storage.
#[derive(Debug)]
pub struct StaticLinkedList<T, const N: usize> {
    /// The array of nodes.
    nodes: [Option<Node<T>>; N],
    /// The index of the head node in the array.
    head: Option<usize>, // Index of the head node in the array
    /// The indices of free slots in the array.
    free: Vec<usize>,    // Indices of free slots in the array
}

impl<T, const N: usize> StaticLinkedList<T, N> {
    /// Creates a new empty StaticLinkedList.
    pub fn new() -> Self {
        let mut free = Vec::with_capacity(N);
        for i in 0..N {
            free.push(i);
        }

        StaticLinkedList {
            nodes: array_init::array_init(|_| None),
            head: None,
            free,
        }
    }

    /// Allocates a new node in the array.
    ///
    /// # Arguments
    ///
    /// * data - The data to be stored in the new node.
    ///
    /// # Returns
    ///
    /// * Some(usize) - The index of the newly allocated node.
    /// * None - If the list is full and no more nodes can be allocated.
    fn allocate_node(&mut self, data: T) -> Option<usize> {
        if self.free.is_empty() {
            return None; // List is full
        }

        let index = self.free.remove(0); // Get the first free index
        self.nodes[index] = Some(Node { data, next: None });
        Some(index)
    }

    /// Deallocates a node in the array.
    ///
    /// # Arguments
    ///
    /// * index - The index of the node to be deallocated.
    fn deallocate_node(&mut self, index: usize) {
        self.nodes[index] = None;
        self.free.push(index);
        self.free.sort_unstable(); // Keep free indices sorted for consistency (optional)
    }
}

impl<T: PartialEq + Clone + Debug, const N: usize> LinkedListTrait<T> for StaticLinkedList<T, N> {
    /// Inserts a new element at the tail of the linked list.
    ///
    /// # Arguments
    ///
    /// * data - The data to be inserted into the linked list.
    fn insert(&mut self, data: T) {
        if let Some(index) = self.allocate_node(data) {
            match self.head {
                None => {
                    self.head = Some(index);
                }
                Some(head_index) => {
                    let mut current_index = head_index;
                    loop {
                        match self.nodes[current_index].as_mut().unwrap().next {
                            None => {
                                self.nodes[current_index].as_mut().unwrap().next = Some(index);
                                break;
                            }
                            Some(next_index) => {
                                current_index = next_index;
                            }
                        }
                    }
                }
            }
        } else {
            println!("StaticLinkedList is full. Cannot insert more elements.");
        }
    }

    /// Inserts a new element at a specified index in the linked list.
    ///
    /// # Arguments
    ///
    /// * index - The index at which the new element should be inserted.
    /// * data - The data to be inserted into the linked list.
    ///
    /// # Returns
    ///
    /// * Ok(()) - If the element was successfully inserted.
    /// * Err(String) - If the index is out of bounds or the list is full.
    fn insert_at_index(&mut self, index: usize, data: T) -> Result<(), String> {
        if index == 0 {
            if let Some(new_index) = self.allocate_node(data) {
                self.nodes[new_index].as_mut().unwrap().next = self.head;
                self.head = Some(new_index);
                return Ok(());
            } else {
                return Err("List is full".to_string());
            }
        }

        let mut current_index = self.head;
        for _ in 0..(index - 1) {
            match current_index {
                Some(i) => {
                    current_index = self.nodes[i].as_ref().unwrap().next;
                }
                None => {
                    return Err("Index out of bounds".to_string());
                }
            }
        }

        match current_index {
            Some(i) => {
                if let Some(new_index) = self.allocate_node(data) {
                    self.nodes[new_index].as_mut().unwrap().next = self.nodes[i].as_mut().unwrap().next;
                    self.nodes[i].as_mut().unwrap().next = Some(new_index);
                    Ok(())
                } else {
                    Err("List is full".to_string())
                }
            }
            None => Err("Index out of bounds".to_string()),
        }
    }

    /// Deletes the first occurrence of an element matching the provided data.
    ///
    /// # Arguments
    ///
    /// * data - The data to be deleted from the linked list.
    ///
    /// # Returns
    ///
    /// * true - If an element was successfully deleted.
    /// * false - If no element matching the data was found.
    fn delete_element(&mut self, data: T) -> bool {
        if self.head.is_none() {
            return false;
        }

        let mut current_index = self.head;

        if self.nodes[self.head.unwrap()].as_ref().unwrap().data == data {
            let head_index = self.head.unwrap();
            self.head = self.nodes[head_index].as_ref().unwrap().next;
            self.deallocate_node(head_index);
            return true;
        }

        while let Some(i) = current_index {
            let next_index = self.nodes[i].as_ref().unwrap().next;
            match next_index {
                Some(j) => {
                    if self.nodes[j].as_ref().unwrap().data == data {
                        self.nodes[i].as_mut().unwrap().next = self.nodes[j].as_ref().unwrap().next;
                        self.deallocate_node(j);
                        return true;
                    } else {
                        current_index = Some(j);
                    }
                }
                None => return false,
            }
        }

        false
    }

    /// Deletes the element at the specified index in the linked list.
    ///
    /// # Arguments
    ///
    /// * index - The index of the element to be deleted.
    ///
    /// # Returns
    ///
    /// * Ok(()) - If the element was successfully deleted.
    /// * Err(String) - If the index is out of bounds.
    fn delete_at_index(&mut self, index: usize) -> Result<(), String> {
        if index == 0 {
            match self.head {
                Some(head_index) => {
                    self.head = self.nodes[head_index].as_ref().unwrap().next;
                    self.deallocate_node(head_index);
                    Ok(())
                }
                None => Err("Index out of bounds".to_string()),
            }
        } else {
            let mut current_index = self.head;
            for _ in 0..(index - 1) {
                match current_index {
                    Some(i) => {
                        current_index = self.nodes[i].as_ref().unwrap().next;
                    }
                    None => return Err("Index out of bounds".to_string()),
                }
            }

            match current_index {
                Some(i) => {
                    match self.nodes[i].as_ref().unwrap().next {
                        Some(j) => {
                            self.nodes[i].as_mut().unwrap().next = self.nodes[j].as_ref().unwrap().next;
                            self.deallocate_node(j);
                            Ok(())
                        }
                        None => Err("Index out of bounds".to_string()),
                    }
                }
                None => Err("Index out of bounds".to_string()),
            }
        }
    }

    /// Updates the first occurrence of an element matching the old_data with new_data.
    ///
    /// # Arguments
    ///
    /// * old_data - The data to be replaced.
    /// * new_data - The new data to replace the old data.
    ///
    /// # Returns
    ///
    /// * true - If an element was successfully updated.
    /// * false - If no element matching the old_data was found.
    fn update_element(&mut self, old_data: T, new_data: T) -> bool {
        let mut current_index = self.head;
        while let Some(i) = current_index {
            if self.nodes[i].as_ref().unwrap().data == old_data {
                self.nodes[i].as_mut().unwrap().data = new_data;
                return true;
            }
            current_index = self.nodes[i].as_ref().unwrap().next;
        }
        false
    }

    /// Updates the element at the specified index with the provided data.
    ///
    /// # Arguments
    ///
    /// * index - The index of the element to be updated.
    /// * data - The new data to update the element with.
    ///
    /// # Returns
    ///
    /// * Ok(()) - If the element was successfully updated.
    /// * Err(String) - If the index is out of bounds.
    fn update_element_at_index(&mut self, index: usize, data: T) -> Result<(), String> {
        let mut current_index = self.head;
        for _ in 0..index {
            match current_index {
                Some(i) => {
                    current_index = self.nodes[i].as_ref().unwrap().next;
                }
                None => return Err("Index out of bounds".to_string()),
            }
        }

        match current_index {
            Some(i) => {
                self.nodes[i].as_mut().unwrap().data = data;
                Ok(())
            }
            None => Err("Index out of bounds".to_string()),
        }
    }

    /// Finds whether an element matching the provided data exists in the list.
    ///
    /// # Arguments
    ///
    /// * data - The data to search for in the linked list.
    ///
    /// # Returns
    ///
    /// * true - If an element matching the data is found.
    /// * false - If no element matching the data is found.
    fn find(&self, data: &T) -> bool {
        let mut current_index = self.head;
        while let Some(i) = current_index {
            if &self.nodes[i].as_ref().unwrap().data == data {
                return true;
            }
            current_index = self.nodes[i].as_ref().unwrap().next;
        }
        false
    }

    /// Retrieves the element at the specified index in the linked list.
    ///
    /// # Arguments
    ///
    /// * index - The index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// * Some(&T) - If an element exists at the specified index.
    /// * None - If the index is out of bounds.
    fn get(&self, index: usize) -> Option<&T> {
        let mut current_index = self.head;
        for _ in 0..index {
            match current_index {
                Some(i) => {
                    current_index = self.nodes[i].as_ref().unwrap().next;
                }
                None => return None,
            }
        }

        match current_index {
            Some(i) => Some(&self.nodes[i].as_ref().unwrap().data),
            None => None,
        }
    }
}