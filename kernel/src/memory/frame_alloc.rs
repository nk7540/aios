use bootloader::boot_info::{MemoryRegionKind, MemoryRegions};
use x86_64::{structures::paging::{PhysFrame, FrameAllocator, Size4KiB}, PhysAddr};

type FrameIterator = impl Iterator<Item = PhysFrame>;

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator {
    usable_frames: FrameIterator,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn init(memory_regions: &'static MemoryRegions) -> Self {
        // get usable regions from memory map
        let regions = memory_regions.iter();
        let usable_regions = regions
            .filter(|r| r.kind == MemoryRegionKind::Usable);
        // map each region to its address range
        let addr_ranges = usable_regions
            .map(|r| r.start..r.end);
        // transform the address ranges into an iterator of frame start addresses, choosing every 4096th address using step_by
        // The bootloader page-aligns all usable memory areas so that we don’t need any alignment or rounding code here.
        // By using flat_map instead of map, we get an Iterator<Item = u64> instead of an Iterator<Item = Iterator<Item = u64>>.
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        let usable_frames = frame_addresses
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)));

        BootInfoFrameAllocator { usable_frames }
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.usable_frames.next()
    }
}
