use std::collections::{HashMap, HashSet};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);

/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    value: T,
}

impl<T> InputCell<T> {
    fn new(initial: T) -> Self {
        Self { value: initial }
    }
}

struct ComputeCell<'a, T> {
    value: T,
    compute_func: Box<dyn Fn(&[T]) -> T>,
    dependencies: Vec<CellId>,
    callbacks: HashMap<CallbackId, Box<dyn Fn(T) + 'a>>,
}

pub struct Reactor<'a, T> {
    input_cells: HashMap<InputCellId, InputCell<T>>,
    compute_cells: HashMap<ComputeCellId, ComputeCell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + Default> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            input_cells: HashMap::new(),
            compute_cells: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let cell_id = InputCellId(self.input_cells.len());
        self.input_cells.insert(cell_id, InputCell::new(initial));
        cell_id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        // Check if the dependencies exist
        for dependency in dependencies {
            let exists = match dependency {
                CellId::Input(id) => self.input_cells.contains_key(&id),
                CellId::Compute(id) => self.compute_cells.contains_key(&id),
            };
            if !exists {
                return Err(*dependency);
            }
        }

        let compute_cell_id = ComputeCellId(self.compute_cells.len());

        // Collect the dependency values so that we can calculate the compute cell's initial value.
        let values: Vec<T> = dependencies
            .iter()
            .map(|id| self.value(*id).unwrap())
            .collect();

        // Finally create the new compute cell
        self.compute_cells.insert(
            compute_cell_id,
            ComputeCell {
                value: compute_func(&values),
                compute_func: Box::new(compute_func),
                dependencies: dependencies.into(),
                callbacks: HashMap::new(),
            },
        );
        Ok(compute_cell_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        Some(match id {
            CellId::Input(id) => self.input_cells.get(&id)?.value,
            CellId::Compute(id) => self.compute_cells.get(&id)?.value,
        })
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let Some(cell) = self.input_cells.get_mut(&id) {
            if cell.value != new_value {
                cell.value = new_value;

                let mut compute_cell_ids = Vec::new();
                for key in self.compute_cells.keys() {
                    compute_cell_ids.push(key.clone());
                }

                let mut callback_ids = HashSet::new();

                for compute_cell_id in compute_cell_ids {
                    let values: Vec<T> = self
                        .compute_cells
                        .get(&compute_cell_id)
                        .unwrap()
                        .dependencies
                        .iter()
                        .map(|id| self.value(*id).unwrap())
                        .collect();

                    self.compute_cells.entry(compute_cell_id).and_modify(|e| {
                        let new_value = (e.compute_func)(&values);
                        if e.value != new_value {
                            e.value = new_value;
                            if e.dependencies.contains(&CellId::Input(id)) {
                                for (callback_id, _callback) in &e.callbacks {
                                    callback_ids.insert(*callback_id);
                                }
                            }
                        }
                    });
                }

                for callback_id in callback_ids {
                    for (_, compute_cell) in &self.compute_cells {
                        if let Some(callback) = compute_cell.callbacks.get(&callback_id) {
                            callback(compute_cell.value);
                        }
                    }
                }
            }
            return true;
        }
        false
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<CB: Fn(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: CB,
    ) -> Option<CallbackId> {
        let compute_cell = self.compute_cells.get_mut(&id)?;
        let callback_id = CallbackId(compute_cell.callbacks.len());
        compute_cell
            .callbacks
            .insert(callback_id, Box::new(callback));
        return Some(callback_id);
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell_id: ComputeCellId,
        callback_id: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let Some(compute_cell) = self.compute_cells.get_mut(&cell_id) else {
            return Err(RemoveCallbackError::NonexistentCell);
        };
        if compute_cell.callbacks.remove(&callback_id).is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        Ok(())
    }
}
