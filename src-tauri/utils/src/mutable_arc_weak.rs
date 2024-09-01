use std::ops::Deref;
use std::sync::{Mutex, Weak};

#[derive(Debug)]
#[allow(dead_code)]
pub struct MutableArcWeak<T> {
    data: Weak<Mutex<T>>,
}

#[allow(dead_code)]
impl<T> MutableArcWeak<T> {
    pub fn set_data(&self, data: T) -> bool {
        if let Some(d) = self.data.upgrade() {
            if let Ok(mut d) = d.lock() {
                *d = data;
            }
            true
        } else {
            false
        }
    }

    pub fn get_data(&self) -> Option<T>
    where
        T: Clone,
    {
        if let Some(d) = self.data.upgrade() {
            if let Ok(da) = d.lock() {
                let v = da.clone();
                return Some(v);
            }
        };
        None
    }

    #[inline]
    pub fn get_data_closure(&self, closure: impl Fn(&T)) -> bool {
        if let Some(d) = self.data.upgrade() {
            if let Ok(da) = d.lock() {
                closure(da.deref());
                return true;
            }
        }
        false
    }
}
