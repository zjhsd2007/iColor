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
Color can also be created in the following ways
```rust
pub fn from_rgb(r: u8, g: u8, b: u8) -> ColorResult<Color>
pub fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> ColorResult<Color>
pub fn from_hsl(h: u32, s: f32, l: f32) -> ColorResult<Color>
pub fn from_hsla(h: u32, s: f32, l: f32, a: f32) -> ColorResult<Color>
pub fn from_hsv(h: u32, s: f32, v: f32) -> ColorResult<Color>
pub fn from_cmyk(c: f32, m: f32, y: f32, k: f32) -> ColorResult<Color>
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

### methods
```rust
//Determine whether the color is a dark color
pub fn is_dark(&self) -> bool
let color = Color::from("#000");
assert!(color.is_dark());

//Determine whether the color is a light color
pub fn is_light(&self) -> bool
let color = Color::from("#fff");
assert!(color.is_light());

// set alpha
pub fn set_alpha(&mut self, alpha: f32) -> &mut Self  
let mut color1 = Color::from_rgb(100,125,25);
assert_eq!(color.to_rgba(), "rgba(100,125,25, 1.0)");
color1.set_alpha(0.4);
assert_eq!(color.to_rgba(), "rgba(100,125,25,0.4)");

// Inverts the color by subtracting each RGB component from 255 and inverting the alpha value.
pub fn negate(&mut self) -> &mut Self 
let mut color1 = Color::from_rgba(25, 125,100, 0.3);
color1.negate();
assert_eq!(color1.to_rgba(), "rgba(200, 100, 125, 0.7)")

//Reduce the alpha value of the color by a given ratio.
pub fn fade(&mut self, ratio: f32) -> &mut Self
let mut color = Color::from("#000").unwrap();
color.fade(0.5);
assert_eq!(color.to_rgba(), "rgba(0,0,0,0.5)");

//Increase the alpha value of the color by a given ratio.
pub fn opaquer(&mut self, ratio: f32) -> &mut Self
let mut color = Color::from_rgba(0,0,0,0.3).unwrap();
color.opaquer(0.5);
assert_eq!(color.to_rgba(), "rgba(0,0,0,0.45)");
```
