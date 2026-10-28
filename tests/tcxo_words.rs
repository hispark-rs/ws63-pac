//! Host-only register-view test; does not access hardware.

#[test]
fn complete_words_and_legacy_accessors_agree() {
    let mut words = [0u32, 0x89ab_cdef, 0x1234_5678, 0xaaaa, 0xbbbb];
    assert_eq!(
        core::mem::size_of_val(&words),
        core::mem::size_of::<ws63_pac::tcxo::RegisterBlock>()
    );
    // SAFETY: exclusively owned, aligned host storage with the exact register
    // block size; the backing array is not accessed while this view is live.
    let regs = unsafe { &*words.as_mut_ptr().cast::<ws63_pac::tcxo::RegisterBlock>() };
    let low = regs.tcxo_count0().read();
    let high = regs.tcxo_count1().read();
    let legacy_low: u16 = low.count0().bits();
    let legacy_high: u16 = high.count1().bits();
    let full_low: u32 = low.count0_full().bits();
    let full_high: u32 = high.count1_full().bits();
    assert_eq!((legacy_low, legacy_high), (0xcdef, 0x5678));
    assert_eq!(
        (u64::from(full_high) << 32) | u64::from(full_low),
        0x1234_5678_89ab_cdef
    );
    assert_eq!(regs.tcxo_count2().read().count2().bits(), 0xaaaa);
    assert_eq!(regs.tcxo_count3().read().count3().bits(), 0xbbbb);
}
