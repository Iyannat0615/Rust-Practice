use std::mem::MaybeUninit;

/// A static, bounded linked list implementation using a fixed-size array of `Option<T>`.
/// 
/// This list is useful when the maximum number of elements (`N`) is known at compile-time.
/// It does not perform dynamic memory allocation and supports basic insert, delete, update,
/// and search operations.
pub struct StaticLinkedList<T, const N: usize> {
    nodes: [Option<T>; N],
    size: usize,
}

impl<T, const N: usize> StaticLinkedList<T, N> {
    /// Creates a new empty `StaticLinkedList` with a capacity of `N`.
    ///
    /// # Returns
    /// A new instance of the list with all slots initialized to `None`.
    pub fn new() -> Self {
        let mut nodes: [MaybeUninit<Option<T>>; N] = unsafe { MaybeUninit::uninit().assume_init() };
    
        for elem in &mut nodes {
            elem.write(None);
        }
    
        let nodes = unsafe {
            // SAFELY transmute only after all elements have been initialized
            std::ptr::read(&nodes as *const _ as *const [Option<T>; N])
        };
    
        StaticLinkedList { nodes, size: 0 }
    }

    /// Inserts a new element at the end of the list.
    ///
    /// # Parameters
    /// - `data`: The value to insert.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err("List is full")` if the list has reached its capacity.
    pub fn insert(&mut self, data: T) -> Result<(), String> {
        if self.size >= N {
            return Err("List is full".to_string());
        }
        self.nodes[self.size] = Some(data);
        self.size += 1;
        Ok(())
    }

    /// Inserts a new element at a specified index, shifting subsequent elements right.
    ///
    /// # Parameters
    /// - `index`: The position to insert at (0-based).
    /// - `data`: The value to insert.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err("Index out of bounds or list is full")` if index is invalid or list is full.
    pub fn insert_at_index(&mut self, index: usize, data: T) -> Result<(), String> {
        if index > self.size || self.size >= N {
            return Err("Index out of bounds or list is full".to_string());
        }

        for i in (index..self.size).rev() {
            self.nodes[i + 1] = self.nodes[i].take();
        }

        self.nodes[index] = Some(data);
        self.size += 1;
        Ok(())
    }

    /// Deletes the first occurrence of the specified element from the list.
    ///
    /// # Parameters
    /// - `data`: The value to remove.
    ///
    /// # Returns
    /// - `true` if the element was found and removed.
    /// - `false` otherwise.
    pub fn delete_element(&mut self, data: T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.size {
            if self.nodes[i].as_ref() == Some(&data) {
                for j in i..(self.size - 1) {
                    self.nodes[j] = self.nodes[j + 1].take();
                }
                self.nodes[self.size - 1] = None;
                self.size -= 1;
                return true;
            }
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
    pub fn delete_at_index(&mut self, index: usize) -> Result<(), String> {
        if index >= self.size {
            return Err("Index out of bounds".to_string());
        }

        for i in index..(self.size - 1) {
            self.nodes[i] = self.nodes[i + 1].take();
        }
        self.nodes[self.size - 1] = None;
        self.size -= 1;
        Ok(())
    }

    /// Returns a reference to the element at the specified index.
    ///
    /// # Parameters
    /// - `index`: The index of the element to retrieve.
    ///
    /// # Returns
    /// - `Some(&T)` if index is valid.
    /// - `None` otherwise.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.size {
            self.nodes[index].as_ref()
        } else {
            None
        }
    }

    /// Checks whether a given value exists in the list.
    ///
    /// # Parameters
    /// - `data`: A reference to the value to find.
    ///
    /// # Returns
    /// - `true` if the value exists in the list.
    /// - `false` otherwise.
    pub fn find(&self, data: &T) -> bool
    where
        T: PartialEq,
    {
        self.nodes[..self.size].iter().any(|node| node.as_ref() == Some(data))
    }

    /// Updates the first occurrence of `old` value with a new value.
    ///
    /// # Parameters
    /// - `old`: The value to replace.
    /// - `new`: The value to insert.
    ///
    /// # Returns
    /// - `true` if an element was updated.
    /// - `false` if the element was not found.
    pub fn update_element(&mut self, old: T, new: T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.size {
            if self.nodes[i].as_ref() == Some(&old) {
                self.nodes[i] = Some(new);
                return true;
            }
        }
        false
    }

    /// Updates the value at a specified index.
    ///
    /// # Parameters
    /// - `index`: The index of the element to update.
    /// - `data`: The new value to set.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err("Index out of bounds")` if the index is invalid.
    pub fn update_element_at_index(&mut self, index: usize, data: T) -> Result<(), String> {
        if index >= self.size {
            return Err("Index out of bounds".to_string());
        }
        self.nodes[index] = Some(data);
        Ok(())
    }
}

impl<T, const N: usize> Default for StaticLinkedList<T, N> {
    /// Provides a default instance of the list using `new()`.
    fn default() -> Self {
        Self::new()
    }
}
