# Design for GenUI Builtin Components

- Components with themes
- Getter and Setter for each `live` prop
- macros for widgets `impl`
- use `features` for release or dev 
- keep `redraw` in user control



# GenUI 内置组件设计

- 带主题的组件: 每个组件都默认带有内置主题, 主题控制组件样式
- 每个 `live` 属性的 Getter 和 Setter: 为每个标记`#[live]`的组件属性都应该有对应的`get`和`set`方法
- 组件 `impl` 的宏：对于组件的`impl`提供大量强大的宏支持来简化代码编写
- 使用 `features` 进行发布或开发: 开发时和发布时由`feature`进行控制，减少体积，明确职责
- 让 `redraw` 保持用户控制: 保持GUI框架特性，让开发者控制何时`redraw`以达到更好的性能
- 平滑统一的动画: 对动画的控制应该保持统一性
- 事件包装: 提供组件事件的包装类型
- 贴近`css`的`prop`: prop的命名和书写规则应贴近`css`并对prop解构，扁平化书写体验
- 现代化组件：强大的可扩展的灵活的组件

## 细节

### render()