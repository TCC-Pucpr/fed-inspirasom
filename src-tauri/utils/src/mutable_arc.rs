use crate::ArcMutex;
use std::sync::{Arc, Mutex, MutexGuard};

/// Wrapper para remover um pouco do codigo necessario para criar e modificar um [Arc]
/// que tenha um mutex
#[derive(Debug)]
pub struct MutableArc<T> {
    data: ArcMutex<T>,
}

impl<T> MutableArc<T> {
    pub fn new(data: T) -> MutableArc<T> {
        Self {
            data: Arc::from(Mutex::new(data)),
        }
    }

    /// Faz lock no valor e seta como o novo valor
    /// Retorna verdadeiro se obteve sucesso, falso caso contrario
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

impl<T> Clone for MutableArc<T> {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data = Arc::clone(&source.data);
    }
}
