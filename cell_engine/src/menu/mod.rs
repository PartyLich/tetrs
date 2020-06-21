use std::fmt;

/// A selectable item in a `Menu`
#[derive(Debug, PartialEq)]
pub struct MenuItem<T>
where
    T: fmt::Debug + PartialEq + Clone,
{
    pub label: &'static str,
    pub event: T,
}

impl<T> MenuItem<T>
where
    T: fmt::Debug + PartialEq + Clone,
{
    pub fn new(label: &'static str, event: T) -> Self {
        MenuItem { label, event }
    }
}

/// A user menu for selecting...things
#[derive(Debug, PartialEq)]
pub struct Menu<T>
where
    T: fmt::Debug + PartialEq + Clone,
{
    selection: usize,
    pub menu_items: Vec<MenuItem<T>>,
}

impl<T> Menu<T>
where
    T: fmt::Debug + PartialEq + Clone,
{
    /// Create a new `Menu` containing the provided `MenuItems`
    pub fn new(menu_items: Vec<MenuItem<T>>) -> Self {
        Menu {
            menu_items,
            selection: 0,
        }
    }

    /// Get the current selection index
    pub fn selection(&self) -> usize {
        self.selection
    }

    /// Modify `current` by `change` within the bounds `min` to `max` (inclusive)
    fn update_selection(&mut self, change: i32) {
        let mut i = change + self.selection as i32;
        i %= self.menu_items.len() as i32;
        i = std::cmp::max(i, 0);
        self.selection = i as usize;
    }

    /// Increment the menu selection
    pub fn inc_selection(&mut self) {
        self.update_selection(1);
    }

    /// Decrement the menu selection
    pub fn dec_selection(&mut self) {
        self.update_selection(-1);
    }

    /// Return domain event for current selection
    pub fn select_item(&self) -> &T {
        &self.menu_items.get(self.selection).unwrap().event
    }
}
