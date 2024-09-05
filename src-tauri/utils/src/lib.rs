use std::error::Error;
use std::sync::{Arc, Mutex};

pub mod mutable_arc;
mod mutable_arc_weak;

pub type GenericResult<T> = Result<T, Box<dyn Error>>;
pub type ArcMutex<P> = Arc<Mutex<P>>;

#[cfg(test)]
mod tests {}