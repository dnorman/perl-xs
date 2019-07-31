type subptr = extern "C" fn(pthx: *mut ::perl_sys::types::PerlInterpreter, cv: *mut crate::raw::CV);

use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

// Not public API. Used by generated code.
#[doc(hidden)]
pub struct Registry  {
    head: AtomicPtr<Node>,
}

struct Node {
    symbol: &'static str,
    ptr: subptr,
    next: Option<&'static Node>,
}

#[doc(hidden)]
impl Registry {
    // Not public API. Used by generated code.
    pub const fn new() -> Self {
        Registry {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }
    pub fn submit(&'static self, symbol: &'static str, ptr: subptr) {
        let new = Box::leak(Box::new(Node { symbol, ptr, next: None }));
        let mut head = self.head.load(Ordering::SeqCst);
        loop {
            let prev = self.head.compare_and_swap(head, new, Ordering::SeqCst);
            if prev == head {
                // Pointer is always null or valid &'static Node.
                new.next = unsafe { prev.as_ref() };
                return;
            } else {
                head = prev;
            }
        }
    }

    pub fn iter (&self) -> Iter {
        let head = self.head.load(Ordering::SeqCst);
        Iter {
            // Head pointer is always null or valid &'static Node.
            node: unsafe { head.as_ref() },
        }
    }
}

pub struct Iter {
    node: Option<&'static Node>,
}

impl Iterator for Iter {
    type Item = (&'static str, &'static subptr);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        let symbol = &node.symbol;
        let ptr = &node.ptr;
        self.node = node.next;
        Some((symbol,ptr))
    }
}