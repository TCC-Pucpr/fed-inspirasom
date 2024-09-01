use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct MutableArc<T> {
    data: Arc<Mutex<T>>,
}

impl<T> MutableArc<T> {
    pub fn new(data: T) -> MutableArc<T> {
        Self {
            data: Arc::from(Mutex::new(data)),
        }
    }

    pub fn set_data(&self, data: T) -> bool {
        if let Ok(mut d) = self.data.lock() {
            *d = data;
            true
        } else {
            false
        }
    }

    pub fn get_data(&self) -> Option<MutexGuard<T>> {
        if let Ok(d) = self.data.lock() {
            Some(d)
        } else {
            None
        }
    }
}

impl<T> From<T> for MutableArc<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> From<Arc<Mutex<T>>> for MutableArc<T> {
    fn from(value: Arc<Mutex<T>>) -> Self {
        Self { data: value }
    }
}
