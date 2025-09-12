# GenUI vs Makepad 原生组件库对比

| 对比维度 | Makepad 原生组件库 | GenUI 增强组件库 |
|---------|-------------------|-------------------|
| **架构设计**<div style="min-width: 60px"></div> | 统一的宏定义性组件 | 在Makepad宏的基础上手动实现WidgetNode，使用更底层的Makepad宏，分离组件样式属性，定义状态与组件属性联系，固定的内置动画实现，使用`Component, SlotComponent` trait增强 |
| **属性系统** | 组件结构体中定义 | 状态驱动的多状态属性系统（like: basic/hover/pressed/disabled），TOML全局配置，上层同步下层，下层强定义覆盖上层，组件中固定使用`prop`表示，使用`Style, SlotStyle, BasicStyle, SlotBasicStyle, ApplyMapImpl, ApplySlotMapImpl`等traits增强实现 |
| **动画系统** | 基本的动画支持，可灵活定义 | 深度集成的动画系统，支持状态自动切换动画，固定动画实现，自定义程度较低 |
| **事件处理** | 标准的事件处理机制 | 增强的事件处理，进行包装 |
| **插槽系统** | 常使用 WidgetRef 作为插槽 | 普通插槽(WidgetRef) + 具名插槽(组件结构体)，通过`SlotDrawer, DrawStep`支持复杂布局，通过`Applys, ApplyMapImpl, ApplySlotMapImpl`等traits进行上下层属性同步 |
| **配置管理** | 无 | 配置优先设计，支持 TOML 配置全局组件 |
| **扩展性** | 可以通过继承扩展，但较为复杂 | 提供了宏和 trait，便于快速创建新组件 |
| **学习曲线** | 需要熟悉 Makepad 的宏系统 | 从使用的角度，几乎没有难度，从开发角度提供了清晰的设计模式，新手更容易上手 |
| **主题支持** | 基础主题支持 `theme_desktop_dark.rs` | 增强的主题系统，支持状态相关的主题配置 `Theme` |
| **组件丰富度** | 基础组件 | 扩展了更多业务组件 |
| **代码复用** | 需要手动复制代码模式 | 提供了宏系统，提高代码复用率 |