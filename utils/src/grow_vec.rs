use std::ops::AddAssign;

/// A vector that grows if we try to access an out-of-bounds index.
struct GrowVec<T>(Vec<T>);

impl<T: Default + AddAssign> GrowVec<T> {
    fn new() -> Self {
        Self(Vec::new())
    }

    // Returns a mutable reference.
    fn get(&mut self, index: usize) -> &mut T {
        if self.0.len() <= index {
            self.0.resize_with(index + 1, T::default);
        }
        &mut self.0[index]
    }
}
