use core::ptr;

use crate::{
    MemoryArea,
    PhysicalAddress,
    VirtualAddress,
};

pub mod emulate;
pub mod x86_64;

pub trait Arch {
    const PAGE_SHIFT: usize;
    const PAGE_ENTRY_SHIFT: usize;
    const PAGE_LEVELS: usize;
    const PAGE_OFFSET: usize;

    const ENTRY_ADDRESS_SHIFT: usize;
    const ENTRY_FLAG_PRESENT: usize;
    const ENTRY_FLAG_WRITABLE: usize;
    const ENTRY_FLAG_USER: usize;
    const ENTRY_FLAG_HUGE: usize;
    const ENTRY_FLAG_GLOBAL: usize;
    const ENTRY_FLAG_NO_EXEC: usize;

    const PAGE_SIZE: usize = 1 << Self::PAGE_SHIFT;
    const PAGE_OFFSET_MASK: usize = Self::PAGE_SIZE - 1;
    const PAGE_ADDRESS_SHIFT: usize = Self::PAGE_LEVELS * Self::PAGE_ENTRY_SHIFT + Self::PAGE_SHIFT;
    const PAGE_ADDRESS_SIZE: usize = 1 << Self::PAGE_ADDRESS_SHIFT;
    const PAGE_ADDRESS_MASK: usize = Self::PAGE_ADDRESS_SIZE - Self::PAGE_SIZE;
    const PAGE_ENTRY_SIZE: usize = 1 << (Self::PAGE_SHIFT - Self::PAGE_ENTRY_SHIFT);
    const PAGE_ENTRIES: usize = 1 << Self::PAGE_ENTRY_SHIFT;
    const PAGE_ENTRY_MASK: usize = Self::PAGE_ENTRIES - 1;
    const PAGE_NEGATIVE_MASK: usize = !(Self::PAGE_ADDRESS_SIZE - 1);

    const ENTRY_ADDRESS_SIZE: usize = 1 << Self::ENTRY_ADDRESS_SHIFT;
    const ENTRY_ADDRESS_MASK: usize = Self::ENTRY_ADDRESS_SIZE - Self::PAGE_SIZE;
    const ENTRY_FLAGS_MASK: usize = !Self::ENTRY_ADDRESS_MASK;

    unsafe fn init() -> &'static [MemoryArea];

    #[inline(always)]
    unsafe fn read<T>(address: VirtualAddress) -> T {
        ptr::read(address.data() as *const T)
    }

    #[inline(always)]
    unsafe fn write<T>(address: VirtualAddress, value: T) {
        ptr::write(address.data() as *mut T, value)
    }

    unsafe fn invalidate(address: VirtualAddress);

    #[inline(always)]
    unsafe fn invalidate_all() {
        Self::set_table(Self::table());
    }

    unsafe fn table() -> PhysicalAddress;

    unsafe fn set_table(address: PhysicalAddress);

    unsafe fn phys_to_virt(phys: PhysicalAddress) -> VirtualAddress {
        VirtualAddress::new(phys.data() + Self::PAGE_OFFSET)
    }
}
