This is a relatively universal color format conversion tool that can convert between #RRGGBB, #RGB, #RRGGBBAA, hsl, hsla, hsv, cmyk.

```rust
use iColor::Color;
let color = Color::from("#ff00aa").unwrap();
let color1 = Color::from("#f0a").unwrap();
let color2 = Color::from("#ff00aa80").unwrap();
let color3 = Color::from("rgb(129, 45, 78)").unwrap();
let color4 = Color::from("rgba(129, 45, 78, 0.8)").unwrap();
let color5 = Color::from("hsl(120, 45%, 90%)").unwrap();
let color6 = Color::from("hsla(120, 45%, 90%, 0.5)").unwrap();
let color7 = Color::from("hsv(120, 60%, 80%)").unwrap();
let color8 = Color::from("cmyk(100, 40, 70, 90)").unwrap();
```
```rust
use iColor::Color;
let mut color = Color::from("#ff00aa").unwrap();
assert_eq!(color.to_hex(), "#FF00AA");
assert_eq!(color.to_rgb(), "rgb(255,0,170)");
assert_eq!(color.to_rgba(), "rgba(255,0,170,1)");
assert_eq!(color.to_hex_alpha(), "#FF00AAFF");
assert_eq!(color.to_alpha_hex(), "#FFFF00AA");
assert_eq!(color.to_hsl(),"hsl(320,100%,50%)");
assert_eq!(color.to_hsla(),"hsla(320,100%,50%,1.0)");
assert_eq!(color.to_hsv(),"hsv(320,100%,100%)");
assert_eq!(color.to_cmyk(),"cmyk(0,100,33,0)");
```