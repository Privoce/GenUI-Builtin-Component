## V0.2.2

### General

- [x] new getter
- [x] new setter
- [x] better macros
- [x] less render
- [x] Use features to distinguish between development and release environments


## V0.2.1

### General

1. `GImage`
   1. remove prop `rotation` (use wrapper view instead)
   2. replace `DrawGView` to `DrawGImage`
   3. load dyn (url, local, base64)
   4. add `set_src()` to replace image
   5. replace src from `LiveDependency` to `Src`      
2. add `getter` and `setter`
   1. `GDropDown`
   2. `GPopup` (`tooltip, popup, dialog, drawer`)
   3. `GTag`
   4. `GToggle`
   5. `GLoading`
   6. `GCollapse`
3. color from `Vec4` -> `String` in `getter` and `setter`
4. `GInput`: fix live render