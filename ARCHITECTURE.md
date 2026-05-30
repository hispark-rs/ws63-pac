# ws63-pac 架构

本仓库是 [ws63-rs](https://github.com/sanchuanhehe/ws63-rs) monorepo 的子模块。

`ws63-pac` 是用 svd2rust 从 [`ws63-svd`](https://github.com/sanchuanhehe/ws63-svd) 生成的 WS63 寄存器访问层，
为 35 个片上外设提供 `RegisterBlock` 与 `Peripherals` 单例。

完整架构与评审（集中维护于主仓库）：
- 组件文档：<https://github.com/sanchuanhehe/ws63-rs/blob/main/docs/architecture/ws63-pac.md>
- 总体架构：<https://github.com/sanchuanhehe/ws63-rs/blob/main/docs/architecture/overview.md>
- 整改排期：<https://github.com/sanchuanhehe/ws63-rs/blob/main/ROADMAP.md>
