use multiboot2::{MemoryAreaIter , MemoryArea};

pub const PAGE_SIZE : usize = 4096;

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame : Frame);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number : usize,
}

impl Frame {
    fn containg_address(address : usize) -> Frame {
        Frame { number : address / PAGE_SIZE }
    }
}

pub struct AreaFrameAllocator {
    next_free_frame : Frame,
    current_area : Option<&'static MemoryArea>,
    areas : MemoryAreaIter,
    kernel_start : Frame,
    kernel_end : Frame,
    multiboot_start : Frame,
    multiboot_end : Frame,
}

impl FrameAllocator for AreaFrameAllocator {

    fn allocate_frame(&mut self) -> Option<Frame> {
        self.current_area.and_then(|area| {
            let frame = Frame { number : self.next_free_frame.number };

            let current_area_last_frame = {
                let address = area.start_address() + area.size() - 1;
                Frame::containg_address(address as usize)
            };

            if frame > current_area_last_frame {
                self.choose_next_area();
            } else if frame >= self.kernel_start && frame <= self.kernel_end {
                self.next_free_frame = Frame {
                    number : self.kernel_end.number + 1
                };
            } else if frame >= self.multiboot_start && frame <= self.multiboot_end {
                self.next_free_frame = Frame {
                    number : self.multiboot_end.number + 1
                };
            } else {
                self.next_free_frame.number += 1;
                return Some(frame);
            }

            self.allocate_frame()
        })
    }

    fn deallocate_frame(&mut self, _frame: Frame) {
        unimplemented!()
    }
}

impl AreaFrameAllocator {

    pub fn new(
        kernel_start : usize,
        kernel_end : usize,
        multiboot_start : usize,
        multiboot_end : usize,
        memory_areas : MemoryAreaIter)
        -> AreaFrameAllocator
    {
        let mut allocator = AreaFrameAllocator {
            next_free_frame : Frame::containg_address(0),
            current_area : None,
            areas : memory_areas,
            kernel_start : Frame::containg_address(kernel_start),
            kernel_end : Frame::containg_address(kernel_end),
            multiboot_start : Frame::containg_address(multiboot_start),
            multiboot_end : Frame::containg_address(multiboot_end),
        };
        allocator.choose_next_area();
        allocator
    }

    fn choose_next_area(&mut self) {
        self.current_area = self.areas
            .clone()
            .filter(|area| {
                let address = area.start_address() + area.size() - 1;
                Frame::containg_address(address as usize) >= self.next_free_frame
            }).min_by_key(|area| area.start_address());

        if let Some(area) = self.current_area {
            let start_frame = Frame::containg_address(area.start_address() as usize);
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }

}

