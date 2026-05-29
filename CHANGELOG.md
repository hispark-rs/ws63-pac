# Changelog

All notable changes to ws63-pac will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- SVD-based peripheral access crate for HiSilicon WS63 (RISC-V RV32IMFC_Zicsr)
- Register block definitions for all 35 on-chip peripherals via svd2rust
- Type-safe `Peripherals` singleton with `take()` and `steal()` access patterns
- External interrupt enum (`ExternalInterrupt`) covering all interrupt sources
- Interrupt vector table (`device.x`) with PROVIDE weak defaults
- Critical section support via `critical-section` feature
- Runtime support via `rt` feature (startup assembly + linker scripts)

### Changed

- `ws63-pac` is a git dependency consumed by `ws63-hal` and `ws63-rt`
