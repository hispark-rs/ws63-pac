# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **SPI_WSR (status register) bit layout** corrected to match the HiSilicon SSI
  v151 silicon (vendor `hal_spi_v151_regs_def.h` `spi_wsr_data`), which spreads the
  flags out instead of packing them like the textbook DesignWare SR:
  `rxfne` = bit 4, `rxff` = bit 5, `txfnf` = bit 11, `txfe` = bit 12,
  `busy` = bit 15, `dcol` = bit 0 (was the wrong `busy` = 0 / `txfnf` = 1 /
  `txfe` = 2 / `rxfne` = 3, plus bogus `rxfo`/`txfo`/`dcm`). With the old layout
  `txfnf` read a reserved bit that is always 0, so the HAL's SPI transfer spun until
  it returned `Timeout` on every call. Reset value updated to 0x1800 (TXFNF|TXFE,
  the idle state). Verified on WS63 silicon 2026-06-14: SPI0 MOSI→MISO HIL loopback
  now round-trips a 4-byte buffer (previously `Err(Timeout)`).
- **TIMER `%s_CONTROL`**: added the `cnt_req` (bit 5, rw) and `cnt_lock` (bit 6, ro)
  fields — the current-count latch handshake. They were missing; reading
  `CURRENT_VALUE` on real silicon needs them (it is a latch, not a live counter),
  so drivers previously had to poke raw bits. Verified against the vendor
  `hal_timer_v150` + on WS63 silicon.
- **TIMER `%s_CONTROL.mode`** enum corrected to the silicon/vendor
  `control_reg_mode_v150_t` values: `OneShot = 0`, `Periodic = 1`, `FreeRun = 3`
  (was the wrong `FreeRunning = 0 / OneShot = 1 / Periodic = 2`).

Regenerated from `ws63-svd/WS63.svd` via `regen.sh` (svd2rust 0.37.1); DMA and the
other blocks are unchanged (the DMA register layout was already silicon-correct —
the earlier DMA bring-up gap was a HAL/QEMU issue, not a PAC one).

## [0.1.3] - 2026-06-02

### Changed

- CI: first release cut by ws63-pac's own repo pipeline (no functional change since 0.1.2).

## [0.1.2] - 2026-06-02

### Added

- Vendored ws63-svd as a nested submodule (the PAC's generation source).

### Changed

- Regenerated from WS63.svd with eFuse/LSADC fixes and reproducible generation pipeline.

### Fixed

- Fixed ws63-pac to compile on non-RISC-V hosts by cfg-gating riscv-specific coupling.

## [0.1.1]

### Added

- KM keyslot registers: `KC_REECPU_LOCK_CMD`, `KC_PCPU_LOCK_CMD`, `KC_AIDSP_LOCK_CMD`,
  `KC_RD_SLOT_NUM` (additive, backwards-compatible API surface — hence the 0.1.1 bump).

### Note

- This 0.1.1 bump exists because public registers were added after the `v0.1.0` tag.
  Re-publishing `0.1.0` with a changed API would violate SemVer; downstream consumers
  (`ws63-hal`) require these registers, so the version is bumped accordingly.

## [0.1.0]

### Added

- SVD-based peripheral access crate for HiSilicon WS63 (RISC-V RV32IMFC_Zicsr)
- Register block definitions for all 35 on-chip peripherals via svd2rust
- Type-safe `Peripherals` singleton with `take()` and `steal()` access patterns
- External interrupt enum (`ExternalInterrupt`) covering all interrupt sources
- Interrupt vector table (`device.x`) with PROVIDE weak defaults
- Critical section support via `critical-section` feature
- Runtime support via `rt` feature (startup assembly + linker scripts)

### Changed

- Consumed by `ws63-hal` and `ws63-rt` as a registry (`version`) dependency; inside the
  `ws63-rs` monorepo it is redirected to the local path via `[patch.crates-io]`.
