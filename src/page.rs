use core::mem::size_of;
use crate::{print, println};

extern "C" {
	static HEAP_START: usize;
	static HEAP_SIZE: usize;
}
static mut ALLOC_START: usize = 0;
static PAGE_ORDER: usize = 12; // 2^12 == 4096
static PAGE_SIZE: usize = 1 << 12;


pub const fn align_val(val: usize, order: usize) -> usize {
	let o = (1usize << order) - 1;
	(val + o) & !o
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

impl Page {
    pub fn is_taken(&self) -> bool {
        self.flags & PageBits::Taken as u8 != 0
    }

    pub fn is_free(&self) -> bool {
        !self.is_taken()
    }

    pub fn is_last(&self) -> bool{
        self.flags & PageBits::Last as u8 != 0
    }

    pub fn set_flag(&mut self, flag: PageBits) {
        self.flags |= flag as u8;
    }

    pub fn clear(&mut self) {
        self.flags = PageBits::Empty as u8;
    }
}

pub fn init() {
    unsafe {
        let num_pages = HEAP_SIZE / PAGE_SIZE;
        let ptr = HEAP_START as *mut Page;

        for i in 0..num_pages {
            (*ptr.add(i)).clear();
        }

        ALLOC_START = align_val(HEAP_START + (num_pages * size_of::<Page>()), PAGE_ORDER);
    }
}

pub fn alloc(pages: usize) -> Option<*mut u8> {
    assert!(pages > 0);

    unsafe {

        // Get max number of page entries
        let num_pages = HEAP_SIZE / PAGE_SIZE;
        let ptr = HEAP_START as *mut Page;

        for i in 0..num_pages - pages {
            if (*ptr.add(i)).is_free() {
                let mut found = true;
                for j in i..i+pages {
                    if (*ptr.add(j)).is_taken() {
                        found = false;
                        break;
                    }
                }

                // Found contiguous section of memory we can create these pages in
                if found {
                    // Mark all page entries as taken
                    for k in i..i+pages-1 {
                        (*ptr.add(k)).set_flag(PageBits::Taken);
                    }
                    // Mark this page as last
                    (*ptr.add(i+pages-1)).set_flag(PageBits::Taken);
                    (*ptr.add(i+pages-1)).set_flag(PageBits::Last);

                    // Return actual memory address that requester can use
                    return Some((ALLOC_START + (PAGE_SIZE * i)) as *mut u8);
                }
            }
        }
    }
    None
}

pub fn dealloc(ptr: *mut u8) {
    assert!(!ptr.is_null());

    unsafe {
        // Calculate page entry
        let page_addr = (ptr as usize - ALLOC_START) / PAGE_SIZE;

        // Not smaller than heap not larger than heap    
        assert!(page_addr >= HEAP_START && page_addr < HEAP_START + HEAP_SIZE);

        let mut p: *mut Page = page_addr as *mut Page;

        while (*p).is_taken() && !(*p).is_last() {
            (*p).clear();
            p = p.add(1);
        }

        assert!((*p).is_last() == false, "Possible double-free detected!");

        (*p).clear();
    }
}

/// Print all page allocations
/// This is mainly used for debugging.
pub fn print_page_allocations() {
	unsafe {
		let num_pages = HEAP_SIZE / PAGE_SIZE;
		let mut beg = HEAP_START as *const Page;
		let end = beg.add(num_pages);
		let alloc_beg = ALLOC_START;
		let alloc_end = ALLOC_START + num_pages * PAGE_SIZE;
		println!();
		println!(
					"PAGE ALLOCATION TABLE\nMETA: {:p} -> {:p}\nPHYS: \
					0x{:x} -> 0x{:x}",
					beg, end, alloc_beg, alloc_end
		);
		println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
		let mut num = 0;
		while beg < end {
			if (*beg).is_taken() {
				let start = beg as usize;
				let memaddr = ALLOC_START
								+ (start - HEAP_START)
								* PAGE_SIZE;
				print!("0x{:x} => ", memaddr);
				loop {
					num += 1;
					if (*beg).is_last() {
						let end = beg as usize;
						let memaddr = ALLOC_START
										+ (end
											- HEAP_START)
										* PAGE_SIZE
										+ PAGE_SIZE - 1;
						print!(
								"0x{:x}: {} page(s)",
								memaddr,
								(end - start + 1)
						);
						println!(".");
						break;
					}
					beg = beg.add(1);
				}
			}
			beg = beg.add(1);
		}
		println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
	}
}		