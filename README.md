# egui_ui_refresh

Changes the default egui fonts, provides new text primitives and tweaks the general style.

New fonts :
- Inter
- Jetbrains Mono

> [!IMPORTANT]
> Using `egui_ui_refresh` will remove all icons.
> Use something like `egui_phosphor` to add icons back.

## Usage

```rust
egui_ctx.set_fonts(egui_ui_refresh::fonts());
egui_ctx.set_style(egui_ui_refresh::style());
```
