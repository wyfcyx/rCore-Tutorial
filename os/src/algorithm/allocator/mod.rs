pub mod stacked_allocator;

pub trait Allocator {
    fn new(capacity: usize) -> Self;
    fn alloc(&mut self) -> Option<usize>;
    fn dealloc(&mut self, index: usize);
}