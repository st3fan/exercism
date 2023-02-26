use core::hash::Hash;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub struct CustomSet<T>
where
    T: Clone + Hash + Eq,
{
    items: HashSet<T>, // "How it works internally doesn't matter, ..."
}

impl<T> CustomSet<T>
where
    T: Clone + Hash + Eq,
{
    pub fn new(input: &[T]) -> Self {
        let mut items: HashSet<T> = HashSet::new();
        for item in input.iter() {
            items.insert(item.clone());
        }
        Self { items }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.items.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.items.contains(&element) {
            self.items.insert(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.items.iter().all(|e| other.items.contains(&e))
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.items.iter().any(|e| other.items.contains(&e))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut set: CustomSet<T> = CustomSet::new(&[]);
        for item in &self.items {
            if other.contains(item) {
                set.add(item.clone())
            }
        }
        set
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut set: CustomSet<T> = CustomSet::new(&[]);

        for item in &self.items {
            if !other.contains(item) {
                set.add(item.clone())
            }
        }

        set
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut set: CustomSet<T> = CustomSet::new(&[]);

        for item in &self.items {
            set.add(item.clone())
        }

        for item in &other.items {
            set.add(item.clone())
        }

        set
    }
}
