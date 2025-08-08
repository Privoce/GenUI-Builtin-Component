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


## prompt

### prop.rs
接下来让我来指导你更改tag的prop.rs, 让我们以tabbar_item的prop作为蓝本，说明和分析一下你写的代码的不足和错误的地方
1. tag和tabbar_item一样都是带有插槽的，tag有左侧的icon插槽，中间的text文字插槽和右侧的关闭图标的插槽，这里的插槽都是具名插槽(具体插槽)，对于有插槽的组件都会使用SlotProp trait处理顶层Prop，也就是impl SlotProp for TagProp，然后会使用component_part!宏来创建组件结构，这里就会含有Container，Icon, Text, Close四个。
2. 插槽组件使用TagBasicProp来声明具体插槽属性，由于这些插槽已经是基本组件了，并且可以直接获取插槽的BasicProp，所以需要让TagBasicProp来实现BasicProp和SlotBasicProp trait
3. set_from_str方法中都可以使用属性类型的from_live_value().unwrap_or来设置，这里我已经改了
4. 你的 TagBasicProp声明是完全错误的，应该像TabbarItemBasicProp那样进行声明

### mod.rs
让我指导你写组件的具体实现mod.rs，我们参考tabbar_item的mod.rs
1. 首先在定义结构体的时候这里selected是不需要的，因为只有disabled需要对应状态，然后是apply_state_map, 对于带有插槽的组件应该使用apply_slot_map，我还看到你声明了一个slot，这个也是不需要的，因为其他icon,text,close已经完整的包含了Tag需要的插槽
2. 然后是WidgetNode这个trait的实现，你只需要记住一个原则，只要不包含GView这样的容器型插槽，那么就可以直接返回WidgetRef::empty()，find_widgets也是，可以直接写个()表示无执行，walk方法直接获取prop的container的walk()即可，因为walk方法就是在设置外层容器，redraw方法则需要对插槽是否可见进行判断，插槽也需要redraw，同时redraw方法前需要使用render方法，但仅限当前组件，意思是不需要调用插槽的render方法。需要补充visible!宏生产需要的方法，需要实现state方法，固定为self.state.to_string()
3. 同样draw_walk, handle_event都可以直接参考
4. 然后就是LiveHook trait需要实现，直接参考就行，实现非常固定
5. 所有组件都需要实现Component trait
6. 带有插槽的组件需要额外实现SlotComponent trait
请对照进行修改
