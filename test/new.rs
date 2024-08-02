use std::alloc::{alloc, Layout};

fn main() {
    std::ptr::null_mut::<i32>();
    let mut alloc = Box::new(unsafe {
        let _ = alloc(Layout::new::<usize>());
    });
    alloc = Box::new(std::ptr::null_mut());
    print!("{:?}", alloc.wrapping_add(0xdacbaffeda))
}
