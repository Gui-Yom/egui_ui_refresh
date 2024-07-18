# egui_ui_refresh

An attempt at improving the egui look and feel. This crate is mainly code pulled from `re_ui`,
the [Rerun](https://rerun.io) UI crate.

This project is very much in development, do not expect anything to work correctly.

## Fonts

This crates packages some fonts :

- Inter for the proportional style
- Jetbrains Mono for the monospace style
- Phosphor icons
- Noto emoji monochrome

You can turn off egui / eframe `default_fonts` crate feature to reduce binary size.

## Usage

```rust
// In the AppCreator closure from eframe
cc.egui_ctx.set_fonts(egui_ui_refresh::fonts::fonts());
RefreshedTheme::init_default().apply( & cc.egui_ctx);
```
