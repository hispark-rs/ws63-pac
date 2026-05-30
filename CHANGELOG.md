# Changelog

All notable changes to ws63-pac will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
