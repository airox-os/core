/// Minimal volatile wrapper for memory-mapped I/O
pub struct Volatile<T> {
    value: T,
}

impl<T> Volatile<T> {
    pub fn new(value: T) -> Self {
        Volatile { value }
    }
    pub fn read(&self) -> T where T: Copy {
        unsafe { core::ptr::read_volatile(&self.value) }
    }
    pub fn write(&mut self, val: T) {
        unsafe { core::ptr::write_volatile(&mut self.value, val) }
    }
}

impl<T> core::ops::Deref for Volatile<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // unsafe { &*self.value }
        &self.value
    }
}

impl<T> core::ops::DerefMut for Volatile<T> {
    fn deref_mut(&mut self) -> &mut T {
        // unsafe { &mut *self.value }
        &mut self.value
    }
}