use super::Allocator;
use alloc::vec::Vec;
use alloc::vec;
pub struct StackedAllocator {
    list: Vec<(usize, usize)>,
}

impl Allocator for StackedAllocator {
    fn new(capacity: usize) -> Self {
        Self {
            list: vec![(0, capacity)],
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        //println!("into StackedAllocator.alloc()");
        if let Some((start, end)) = self.list.pop() {
            if end - start > 1 {
                self.list.push((start + 1, end))
            }
            //println!("alloc = {}", start);
            Some(start)
        } else {
            None
        }
    }

    fn dealloc(&mut self, idx: usize) {
        self.list.push((idx, idx + 1));
    }
}

