use std::{arch::asm, hint::unreachable_unchecked};

#[inline(always)]
pub fn assume(p: bool) {
    debug_assert!(p);
    if !p {
        unsafe {
            unreachable_unchecked();
        }
    }
}

#[inline(always)]
pub fn branch_hint() {
    unsafe {
        asm!("", options(nomem, nostack, preserves_flags));
    }
}
