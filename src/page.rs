
extern "C" {
	static HEAP_START: usize;
	static HEAP_SIZE: usize;
}

#[repr(u8)]
pub enum PageBits {
    Empty = 0,
    Taken = 1 << 0,
    Last = 1 << 1,
}

pub struct Page {
    flags: u8,
}

pub fn alloc(pages: usize) -> *mut u8 {
    assert!(pages > 0);
    unsafe {

    }
}