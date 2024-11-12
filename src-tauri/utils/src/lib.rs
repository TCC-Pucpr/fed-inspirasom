use std::sync::{Arc, Mutex};

pub mod mutable_arc;
mod mutable_arc_weak;

pub type ArcMutex<P> = Arc<Mutex<P>>;

#[cfg(test)]
mod tests {}
