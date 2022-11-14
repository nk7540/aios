use x86_64::{
    structures::paging::PageTable,
    VirtAddr, PhysAddr,
};

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    // First, we read the physical frame of the active level 4 table from the CR3 register.
    let (level_4_table_frame, _) = Cr3::read();

    // We then take its physical start address, convert it to a u64,
    // and add it to physical_memory_offset to get the virtual address where the page table frame is mapped.
    // Finally, we convert the virtual address to a *mut PageTable raw pointer through the as_mut_ptr method 
    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    // and then unsafely create a &mut PageTable reference from it.
    &mut *page_table_ptr // unsafe
}
