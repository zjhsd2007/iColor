//! This is a relatively universal color format conversion tool that can convert between #RRGGBB, #RGB, #RRGGBBAA, hsl, hsla, hsv, cmyk.

mod utils;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Color(u8, u8, u8, f32);

#[derive(Debug, Clone, Copy)]
pub enum ColorError {
    Format,
    Value,
}

type ColorResult<T> = Result<T, ColorError>;

const HEX_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#(\w{2})(\w{2})(\w{2})$").unwrap());
const HEX_WITH_TRANS_REG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^#(\w{2})(\w{2})(\w{2})(\w{2})$").unwrap());
const SHORT_HEX_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#(\w)(\w)(\w)$").unwrap());
const RGB_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^rgb\((\d+),(\d+),(\d+)\)$").unwrap());
const RGBA_REG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^rgba\((\d+),(\d+),(\d+),(\d+(\.\d+)?)\)$").unwrap());
const HSL_REG:Lazy<Regex> = Lazy::new(|| Regex::new(r"^hsl\((\d+),(\d+)%,(\d+)%\)$").unwrap());
const HSLA_REG:Lazy<Regex> = Lazy::new(|| Regex::new(r"^hsla\((\d+),(\d+)%,(\d+)%,(0\.\d+)\)$").unwrap());
const CMYK_REG:Lazy<Regex> = Lazy::new(|| Regex::new(r"^cmyk\((\d+),(\d+),(\d+),(\d+)\)$").unwrap());
const HSV_REG:Lazy<Regex> = Lazy::new(|| Regex::new(r"^hsv\((\d+),(\d+)%,(\d+)%\)$").unwrap());

impl Color {
    /// create Color from str.
    /// ## Arguments
    /// * `color_str` - Specify the color, ex: `#FF00AA`, `#FF00AA80`, `rgb(129,45,78)`, `rgba(129,45,78, 0.8)`, `hsl(120, 45%, 90%)`, `hsla(120, 45%, 90%, 0.5)`, `hsv(120, 60%, 80%)`, `cmyk(100,40,70,90)` not case sensitive.
    /// ## Return
    /// * `ColorResult<Color>`, if the `color_str` format is invalid, it will be return ColorError::Format error, else return Color
    /// ## Example
    /// ``` rust
    /// use iColor::Color;
    /// let color = Color::from("#ff00aa").unwrap();
    /// let color1 = Color::from("#f0a").unwrap();
    /// let color2 = Color::from("#ff00aa80").unwrap();
    /// let color3 = Color::from("rgb(129, 45, 78)").unwrap();
    /// let color4 = Color::from("rgba(129, 45, 78, 0.8)").unwrap();
    /// let color5 = Color::from("hsl(120, 45%, 90%)").unwrap();
    /// let color6 = Color::from("hsla(120, 45%, 90%, 0.5)").unwrap();
    /// let color7 = Color::from("hsv(120, 60%, 80%)").unwrap();
    /// let color8 = Color::from("cmyk(100, 40, 70, 90)").unwrap();
    /// ```
    /// 
    pub fn from(color: &str) -> ColorResult<Color> {
        let len = color.len();
        if color.starts_with('#') {
            // #RRGGBB || #RGB
            if len == 4 || len == 7 {
                return Color::from_hex(color);
            }
            // #RRGGBBAA
            if len == 9 {
                return Color::from_hex_alpha(color);
            }
        }

        let color_str = color.replace(" ", "");
        // rgb string
        if color_str.starts_with("rgb(") {
            return Color::from_rgb_str(color_str.as_str());
        }

        // rgba string
        if color_str.starts_with("rgba(") {
            return Color::from_rgba_str(color_str.as_str());
        }

        // hsl string
        if color_str.starts_with("hsl(") {
            return Color::from_hsl_str(color_str.as_str());
        }

        // hsla string
        if color_str.starts_with("hsla") {
            return Color::from_hsla_str(color_str.as_str());
        }

        // hsv string
        if color_str.starts_with("hsv(") {
            return Color::from_hsv_str(color_str.as_str());
        }

        // cmyk string
        if color_str.starts_with("cmyk(") {
            return Color::from_cmyk_str(color_str.as_str());
        }

        Err(ColorError::Format)
    }

    /// Parses a hexadecimal color string and returns a `Color` instance.
    /// 
    /// # Arguments
    /// 
    /// * `hex` - A hexadecimal color string in the format of "#RRGGBB" or "#RGB".
    /// 
    /// # Returns
    /// 
    /// A `Color` instance if the input string is a valid hexadecimal color string, otherwise a `ColorError::Format` error.
    pub fn from_hex(hex:&str) -> ColorResult<Color> {
        if let Some(cps) = HEX_REG
            .captures(hex)
            .or_else(|| SHORT_HEX_REG.captures(hex))
        {
            let r = utils::match_to_num(cps.get(1).as_ref());
            let g = utils::match_to_num(cps.get(2).as_ref());
            let b = utils::match_to_num(cps.get(3).as_ref());
            return match (r, g, b) {
                (Some(r), Some(g), Some(b)) => Ok(Color(r, g, b, 1.0)),
                _ => Err(ColorError::Format),
            };
        }
        Err(ColorError::Format)
    }

    /// Parses a hexadecimal color string with alpha channel and returns a `Color` instance.
    /// 
    /// # Arguments
    /// 
    /// * `hex_alpha` - A hexadecimal color string with alpha channel in the format of "#RRGGBBAA".
    /// 
    /// # Returns
    /// 
    /// A `Color` instance if the input string is a valid hexadecimal color string with alpha channel, otherwise a `ColorError::Format` error.
    pub fn from_hex_alpha(hex_alpha:&str) -> ColorResult<Color> {

        if let Some(cps) = HEX_WITH_TRANS_REG.captures(hex_alpha) {
            let r = utils::match_to_num(cps.get(1).as_ref());
            let g = utils::match_to_num(cps.get(2).as_ref());
            let b = utils::match_to_num(cps.get(3).as_ref());
            let a = utils::match_to_num2(cps.get(4).as_ref()).map(|v| (v / 255_u8) as f32);
            return match (r, g, b, a) {
                (Some(r), Some(g), Some(b), Some(a)) => Ok(Color(r, g, b, a)),
                _ => Err(ColorError::Format),
            };
        }
        Err(ColorError::Format)
    }

    /// Parses a string in the format of "rgb(R,G,B)" and returns a `Color` instance.
    /// 
    /// # Arguments
    /// 
    /// * `rgb` - A string in the format of "rgb(R,G,B)".
    /// 
    /// # Returns
    /// 
    /// A `Color` instance if the input string is a valid RGB string, otherwise a `ColorError::Format` error.
    pub fn from_rgb_str(rgb:&str) -> ColorResult<Color> {
        if let Some(cps) = RGB_REG.captures(rgb) {
            let r = utils::match_to_num2(cps.get(1).as_ref());
            let g = utils::match_to_num2(cps.get(2).as_ref());
            let b = utils::match_to_num2(cps.get(3).as_ref());
            return match (r, g, b) {
                (Some(r), Some(g), Some(b)) => Ok(Color(r, g, b, 1.0)),
                _ => Err(ColorError::Format),
            };
        }
        Err(ColorError::Format)
    }

    /// Parses a string in the format of "rgba(R,G,B,A)" and returns a `Color` instance.
    /// 
    /// # Arguments
    /// 
    /// * `rgba` - A string in the format of "rgba(R,G,B,A)".
    /// 
    /// # Returns
    /// 
    /// A `Color` instance if the input string is a valid RGB string, otherwise a `ColorError::Format` error.
    pub fn from_rgba_str(rgba:&str) -> ColorResult<Color> {
        if let Some(cps) = RGBA_REG.captures(rgba) {
            let r = utils::match_to_num2(cps.get(1).as_ref());
            let g = utils::match_to_num2(cps.get(2).as_ref());
            let b = utils::match_to_num2(cps.get(3).as_ref());
            let a = cps.get(3).and_then(|v| v.as_str().parse::<f32>().ok());
            return match (r, g, b, a) {
                (Some(r), Some(g), Some(b), Some(a)) => Ok(Color(r, g, b, a)),
                _ => Err(ColorError::Format),
            };
        }
        Err(ColorError::Format)
    }

    /// Parses a string in the format of "hsl(H,S,L)" and returns a `Color` instance.
    /// 
    /// # Arguments
    /// 
    /// * `hsl` - A string in the format of "hsl(H,S,L)".
    /// 
    /// # Returns
    /// 
    /// A `Color` instance if the input string is a valid RGB string, otherwise a `ColorError::Format` error.
    pub fn from_hsl_str(hsl:&str) -> ColorResult<Color> {
        if let Some(cps) = HSL_REG.captures(hsl) {
            let h = cps.get(1).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            let s = cps.get(2).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            let l = cps.get(3).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            return match (h, s ,l) {
                (Some(h), Some(s), Some(l)) => Color::from_hsl(h,s as f32 / 100.0,l as f32 / 100.0),
                _ => Err(ColorError::Format)
            };
        }
        Err(ColorError::Format)
    }

    /// Parses a string in the format of "hsla(H,S,L,A)" and returns a `Color` instance.
    /// 
    /// # Arguments
    /// 
    /// * `hsla` - A string in the format of "hsla(H,S,L,A)".
    /// 
    /// # Returns
    /// 
    /// A `Color` instance if the input string is a valid RGB string, otherwise a `ColorError::Format` error.
    pub fn from_hsla_str(hsla:&str) -> ColorResult<Color> {
        if let Some(cps) = HSLA_REG.captures(hsla) {
            let h = cps.get(1).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            let s = cps.get(2).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            let l = cps.get(3).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            let a = cps.get(4).map(|c| c.as_str()).and_then(|s| s.parse::<f32>().ok());
            return match (h, s ,l, a) {
                (Some(h), Some(s), Some(l), Some(a)) => Color::from_hsla(h,s as f32 / 100.0,l as f32 / 100.0, a),
                _ => Err(ColorError::Format)
            };
        }
        Err(ColorError::Format)
    }

    /// Parses a string in the format of "hsv(H,S,V)" and returns a `Color` instance.
    /// 
    /// # Arguments
    /// 
    /// * `hsv` - A string in the format of "hsv(H,S,V)".
    /// 
    /// # Returns
    /// 
    /// A `Color` instance if the input string is a valid RGB string, otherwise a `ColorError::Format` error.
    pub fn from_hsv_str(hsv:&str) -> ColorResult<Color> {
        if let Some(cps) = HSV_REG.captures(hsv) {
            let h = cps.get(1).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            let s = cps.get(2).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            let v = cps.get(3).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            return match (h, s ,v) {
                (Some(h), Some(s), Some(v)) => Color::from_hsv(h,s as f32 / 100.0,v as f32 / 100.0),
                _ => Err(ColorError::Format)
            };
        }
        Err(ColorError::Format)
    }

    /// Parses a string in the format of "cmyk(C,M,Y,K)" and returns a `Color` instance.
    /// 
    /// # Arguments
    /// 
    /// * `cmyk` - A string in the format of "cmyk(C,M,Y,K)".
    /// 
    /// # Returns
    /// 
    /// A `Color` instance if the input string is a valid RGB string, otherwise a `ColorError::Format` error.
    pub fn from_cmyk_str(cmyk:&str) -> ColorResult<Color> {
        if let Some(cps) = CMYK_REG.captures(cmyk) {
            let c = cps.get(1).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            let m = cps.get(2).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            let y = cps.get(3).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            let k = cps.get(4).map(|c| c.as_str()).and_then(|s| s.parse::<u32>().ok());
            return match (c,m,y,k) {
                (Some(c), Some(m), Some(y), Some(k)) => Color::from_cmyk(c as f32 / 100.0,m as f32 / 100.0,y as f32 / 100.0, k as f32 / 100.0),
                _ => Err(ColorError::Format)
            };
        }
        Err(ColorError::Format)
    }

    /// create Color from hsl
    /// ## Arguments
    /// * h  - Specify the Hue, the value need be between in 0 - 360
    /// * s  - Specify the Saturation, the value need be between in 0.0 - 1.0
    /// * l  - Specify teh Lightness, the value need be between in 0.0 - 1.0
    /// ## Example
    /// ``` rust
    /// use iColor::Color;
    /// let color = Color::from_hsl(210, 0.79, 0.3).unwrap();
    /// assert_eq!(color.to_hex(), "#104C88");
    /// ```
    pub fn from_hsl(h: u32, s: f32, l: f32) -> ColorResult<Color> {
        if !utils::is_valid_num(&s) || !utils::is_valid_num(&l) || !(0..360).contains(&h) {
            return Err(ColorError::Value);
        }
        let c = (1.0 - (l * 2.0 - 1.0).abs()) * s;
        let x = c * (1.0 - ((h as f32 / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;
        let (mut r, mut g, mut b) = match h {
            n if n < 60 => (c, x, 0.0),
            n if 60 <= n && n < 120 => (x, c, 0.0),
            n if 120 <= n && n < 180 => (0.0, c, x),
            n if 180 <= n && n < 240 => (0.0, x, c),
            n if 240 <= n && n < 300 => (x, 0.0, c),
            n if 300 <= n && n < 360 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };
        r = (r + m) * 255.0;
        g = (g + m) * 255.0;
        b = (b + m) * 255.0;
        Ok(Color(r as u8, g as u8, b as u8, 1.0))
    }

    /// create Color from hsla
    /// ## Arguments
    /// * h  - Specify the Hue, the value need be between in 0 - 360
    /// * s  - Specify the Saturation, the value need be between in 0.0 - 1.0
    /// * l  - Specify teh Lightness, the value need be between in 0.0 - 1.0
    /// * a  - Specify the Alpha, the value need be between in 0.0 - 1.0
    /// ## Example
    /// ``` rust
    /// use iColor::Color;
    /// let color = Color::from_hsla(210, 0.79, 0.3, 0.5).unwrap();
    /// assert_eq!(color.to_hex(), "#87A5C3");
    /// ```
    pub fn from_hsla(h: u32, s: f32, l: f32, a: f32) -> ColorResult<Color> {
        let mut color = Color::from_hsl(h, s, l)?;
        color.set_alpha(a);
        Ok(color)
    }

    /// create Color from rgb
    /// ## Arguments
    /// * r  - Specify the Red, the value need be between in 0 - 255
    /// * g  - Specify the Green, the value need be between in 0 - 255
    /// * b  - Specify the Blue, the value need be between in 0 - 255
    /// ## Example
    /// ``` rust
    /// use iColor::Color;
    /// let color = Color::from_rgb(16, 76, 136).unwrap();
    /// assert_eq!(color.to_hex(), "#104C88");
    /// 
    pub fn from_rgb(r: u8, g: u8, b: u8) -> ColorResult<Color> {
        Ok(Color(r, g, b, 1.0))
    }

    /// create Color from rgba
    /// ## Arguments
    /// * r  - Specify the Red, the value need be between in 0 - 255
    /// * g  - Specify the Green, the value need be between in 0 - 255
    /// * b  - Specify the Blue, the value need be between in 0 - 255
    /// * a  - Specify the Alpha, the value need be between in 0.0 - 1.0
    /// ## Example
    /// ``` rust
    /// use iColor::Color;
    /// let color = Color::from_rgba(16, 76, 136, 0.5).unwrap();
    /// assert_eq!(color.to_hex(), "#87A5C3");
    /// 
    pub fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> ColorResult<Color> {
        return if !utils::is_valid_num(&a) {
            Err(ColorError::Value)
        } else {
            Ok(Color(r, g, b, a))
        }      
    }

    /// create Color from cmyk
    /// ## Arguments
    /// * c  - Specify the Cyan, the value need be between in 0.0 - 1.0
    /// * m  - Specify the Magenta, the value need be between in 0.0 - 1.0
    /// * y  - Specify the Yellow, the value need be between in 0.0 - 1.0
    /// * k  - Specify the Key (Black), the value need be between in 0.0 - 1.0
    /// ## Example
    /// ``` rust
    /// use iColor::Color;
    /// let color = Color::from_cmyk(0.5, 0.2, 0.1, 0.1).unwrap();
    /// assert_eq!(color.to_hex(), "#72B7CE");
    /// 
    pub fn from_cmyk(c: f32, m: f32, y: f32, k: f32) -> ColorResult<Color> {
        if !utils::is_valid_num(&c)
            || !utils::is_valid_num(&m)
            || !utils::is_valid_num(&y)
            || !utils::is_valid_num(&k)
        {
            return Err(ColorError::Value);
        }
        let t = 1.0 - k;
        let r = (1.0 - c) * t * 255.0;
        let g = (1.0 - m) * t * 255.0;
        let b = (1.0 - y) * t * 255.0;
        Ok(Color(r as u8, g as u8, b as u8, 1.0))
    }

    /// create Color from hsv
    /// ## Arguments
    /// * h  - Specify the Hue, the value need be between in 0 - 360
    /// * s  - Specify the Saturation, the value need be between in 0.0 - 1.0
    /// * v  - Specify the Value, the value need be between in 0.0 - 1.0
    /// ## Example
    /// ``` rust
    /// use iColor::Color;
    /// let color = Color::from_hsv(210, 0.44, 0.8).unwrap();
    /// assert_eq!(color.to_hex(), "#729FCC");
    /// 
    pub fn from_hsv(h: u32, s: f32, v: f32) -> ColorResult<Color> {
        if !utils::is_valid_num(&s) || !utils::is_valid_num(&v) || !(0..360).contains(&h) {
            return Err(ColorError::Value);
        }
        let c = v * s;
        let x = c * (1.0 - ((h as f32 / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        let (mut r, mut g, mut b) = match h {
            n if n < 60 => (c, x, 0.0),
            n if 60 <= n && n < 120 => (x, c, 0.0),
            n if 120 <= n && n < 180 => (0.0, c, x),
            n if 180 <= n && n < 240 => (0.0, x, c),
            n if 240 <= n && n < 300 => (x, 0.0, c),
            n if 300 <= n && n < 360 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };
        r = (r + m) * 255.0;
        g = (g + m) * 255.0;
        b = (b + m) * 255.0;
        Ok(Color(r as u8, g as u8, b as u8, 1.0))
    }

    /// Convert the color to a hexadecimal string representation.
    /// If the alpha channel is not 1.0, it will be computed with reg, green, and blue.
    /// 
    /// # Example
    /// ``` rust
    /// use iColor::Color;
    /// let color = Color::from_rgb(255, 0, 0).unwrap();
    /// assert_eq!(color.to_hex(), "#FF0000");
    /// 
    /// let color2 = Color::from_rgba(0,0,0,0.5).unwrap();
    /// assert_eq!(color2.to_hex(), "#7F7F7F");
    /// ```
    
    pub fn to_hex(&self) -> String {
        let r = utils::calc_rgb_with_alpha(self.0, self.3) as u8;
        let g = utils::calc_rgb_with_alpha(self.1, self.3) as u8;
        let b = utils::calc_rgb_with_alpha(self.2, self.3) as u8;
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Convert the color to a hexadecimal string with alpha representation.
    /// ```
    /// use iColor::Color;
    /// let color = Color::from_rgba(255, 0, 0, 0.5).unwrap();
    /// assert_eq!(color.to_hex_alpha(), "#FF00007F");
    /// ```
    pub fn to_hex_alpha(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            self.0,
            self.1,
            self.2,
            (self.3 * 255.0) as u8
        )
    }

    /// Convert the color to the format required by Excel, where the color format is usually #AARRGGBB, where AA is alpha
    /// ```
    /// use iColor::Color;
    /// let color = Color::from("#FF0000").unwrap();
    /// assert_eq!(color.to_alpha_hex(), "#FFFF0000");
    /// 
    /// let mut color2 = Color::from("#000").unwrap();
    /// color2.set_alpha(0.5);
    /// assert_eq!(color2.to_alpha_hex(), "#7F000000");
    /// ```
    pub fn to_alpha_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            (self.3 * 255.0) as u8,
            self.0,
            self.1,
            self.2
        )
    }

    /// Convert the color to a CSS RGB string representation.
    /// If the alpha channel is not 1.0, it will be computed with red, green, and blue.
    /// 
    /// # Example
    /// ```rust
    /// use iColor::Color;
    /// let color = Color::from("#FF0000").unwrap();
    /// assert_eq!(color.to_rgb(), "rgb(255,0,0)");
    /// 
    /// let mut color2 = Color::from("#000").unwrap();
    /// color2.set_alpha(0.5);
    /// assert_eq!(color2.to_rgb(), "rgb(127,127,127)");
    /// ```
    pub fn to_rgb(&self) -> String {
        let r = utils::calc_rgb_with_alpha(self.0, self.3) as u8;
        let g = utils::calc_rgb_with_alpha(self.1, self.3) as u8;
        let b = utils::calc_rgb_with_alpha(self.2, self.3) as u8;
        format!("rgb({},{},{})", r, g, b)
    }

    /// Convert the color to a CSS RGBA string representation.
    /// 
    /// # Example
    /// ```rust
    /// use iColor::Color;
    /// let color = Color::from("#FF0000").unwrap();
    /// assert_eq!(color.to_rgba(), "rgba(255,0,0,1)");
    /// 
    /// let mut color2 = Color::from("#000").unwrap();
    /// color2.set_alpha(0.5);
    /// assert_eq!(color2.to_rgba(), "rgba(0,0,0,0.5)");
    /// ```
    pub fn to_rgba(&self) -> String {
        format!("rgba({},{},{},{})", self.0, self.1, self.2, self.3)
    }

    fn to_hsl_val(&self, with_alpha:bool) -> (u32, f32, f32) {
        let (r, g, b) = if with_alpha {
            (
                utils::calc_rgb_with_alpha(self.0, self.3) / 255.0,
                utils::calc_rgb_with_alpha(self.1, self.3) / 255.0,
                utils::calc_rgb_with_alpha(self.2, self.3) / 255.0
            )
        } else {
            (
                self.0 as f32 / 255.0,
                self.1 as f32 / 255.0,
                self.2 as f32 / 255.0
            )
        };

        let c_max = r.max(g).max(b);
        let c_min = r.min(g).min(b);
        let delta = c_max - c_min;
        let mut h = if delta == 0.0 {
            0.0
        } else if c_max == r {
            60.0 * ((g - b) / delta % 6.0)
        } else if c_max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };
        if h < 0.0 {
            h += 360.0;
        }
        let l = (c_max + c_min) / 2.0;
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };
        (h.round() as u32, s, l)
    }

    /// Convert the color to a CSS HSL string representation.
    /// If the alpha channel is not 1.0, it will be computed with red, green, and blue.
    /// 
    /// # Example
    /// ```rust
    /// use iColor::Color;
    /// let color = Color::from("#FF0000").unwrap();
    /// assert_eq!(color.to_hsl(), "hsl(0,100%,50%)");
    /// 
    /// let mut color2 = Color::from("#000").unwrap();
    /// color2.set_alpha(0.5);
    /// assert_eq!(color2.to_hsl(), "hsl(0,0%,50%)");
    /// ```
    pub fn to_hsl(&self) -> String {
        let (h, s, l) = self.to_hsl_val(true);
        format!("hsl({:.0},{:.0}%,{:.0}%)", h, s*100.0, l*100.0)
    }

    /// Convert the color to a CSS HSLA string representation.A
    /// ```
    /// use iColor::Color;
    /// let color = Color::from("#FF0000").unwrap();
    /// assert_eq!(color.to_hsla(), "hsla(0,100%,50%,1.0)");
    /// 
    /// let mut color2 = Color::from("#000").unwrap();
    /// color2.set_alpha(0.5);
    /// assert_eq!(color2.to_hsla(), "hsla(0,0%,0%,0.5)");
    /// ```
    pub fn to_hsla(&self) -> String {
        let (h, s, l) = self.to_hsl_val(false);
        format!("hsla({:.0},{:.0}%,{:.0}%,{:.1})", h, s*100.0, l*100.0, self.3)
    }

    /// Convert the color to a CSS HSLA string representation.A
    /// ```rust
    /// use iColor::Color;
    /// let color = Color::from("#FF0000").unwrap();
    /// assert_eq!(color.to_hsv(), "hsv(0,100%,100%)");
    /// 
    /// let mut color2 = Color::from("#000").unwrap();
    /// color2.set_alpha(0.5);
    /// assert_eq!(color2.to_hsv(), "hsv(0,0%,50%)");
    /// ```
    pub fn to_hsv(&self) -> String {
        let r = utils::calc_rgb_with_alpha(self.0, self.3) / 255.0;
        let g = utils::calc_rgb_with_alpha(self.1, self.3) / 255.0;
        let b = utils::calc_rgb_with_alpha(self.2, self.3) / 255.0;

        let c_max = r.max(g).max(b);
        let c_min = r.min(g).min(b);
        let delta = c_max - c_min;

        let mut h = if delta == 0.0 {
            0.0
        } else if c_max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if c_max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        if h < 0.0 {
            h += 360.0;
        }

        let s = if c_max == 0.0 { 0.0 } else { delta / c_max };

        let v = c_max;

        format!("hsv({:.0},{:.0}%,{:.0}%)", h, s*100.0, v*100.0)
    }

    /// Convert the color to a CSS cmyk string representation.A
    /// ```rust
    /// use iColor::Color;
    /// let color = Color::from("#FF0000").unwrap();
    /// assert_eq!(color.to_cmyk(), "cmyk(0,100,100,0)");
    /// 
    /// let mut color2 = Color::from("#000").unwrap();
    /// color2.set_alpha(0.5);
    /// assert_eq!(color2.to_cmyk(), "cmyk(0,0,0,50)");
    /// ```
    pub fn to_cmyk(&self) -> String {
        let r = utils::calc_rgb_with_alpha(self.0, self.3) / 255.0;
        let g = utils::calc_rgb_with_alpha(self.1, self.3) / 255.0;
        let b = utils::calc_rgb_with_alpha(self.2, self.3) / 255.0;

        let k = 1.0 - r.max(g).max(b);
        let (c,m,y) = if k == 1.0 { (0.0, 0.0, 0.0)} else {
            (
                (1.0 - r - k) / (1.0 - k),
                (1.0 - g - k) / (1.0 - k),
                (1.0 - b - k) / (1.0 - k)
            )
        };
        format!("cmyk({:.0},{:.0},{:.0},{:.0})", c*100.0, m*100.0, y*100.0, k*100.0)
    }

    /// Set the alpha value of the color.
    ///
    /// # Arguments
    ///
    /// * `alpha` - A float value between 0.0 and 1.0 representing the alpha value of the color.
    ///
    /// # Example
    ///
    /// ```
    /// use iColor::Color;
    ///
    /// let mut color = Color::from("#000").unwrap();
    /// color.set_alpha(0.5);
    /// assert_eq!(color.to_hsl(), "hsl(0,0%,50%)");
    /// 
    pub fn set_alpha(&mut self, alpha: f32) -> &mut Self {
        self.3 = alpha;
        self
    }

    /// Determine whether the color is a dark color
    pub fn is_dark(&self) -> bool {
        let (_,_,l) = self.to_hsl_val(true);
        l < 0.5
    }

    /// Determine whether the color is a light color
    pub fn is_light(&self) -> bool {
        !self.is_dark()
    }

    /// Inverts the color by subtracting each RGB component from 255 and inverting the alpha value.
    pub fn negate(&mut self) -> &mut Self {
        self.0 = 255 - self.0;
        self.1 = 255 - self.1;
        self.2 = 255 - self.2;
        self.3 = 1.0 - self.3;
        self
    }

    /// Reduce the alpha value of the color by a given ratio.
    /// # Arguments
    /// * `ratio` - A float value between 0.0 and 1.0 representing the ratio by which to reduce the alpha value.
    /// # Example
    /// ```
    /// use iColor::Color;
    ///
    /// let mut color = Color::from("#000").unwrap();
    /// color.fade(0.5);
    /// assert_eq!(color.to_rgba(), "rgba(0,0,0,0.5)");
    /// 
    /// color.fade(0.5);
    /// assert_eq!(color.to_rgba(), "rgba(0,0,0,0.25)");
    /// ``` 
    pub fn fade(&mut self, ratio: f32) -> &mut Self {
        let ratio = ratio.max(0.0).min(1.0);
        self.3 = ((self.3 - self.3 * ratio) * 100.0).round() / 100.0;
        self
    }

    /// Increase the alpha value of the color by a given ratio.
    ///
    /// # Arguments
    /// * `ratio` - A float value between 0.0 and 1.0 representing the ratio by which to increase the alpha value.
    /// # Example
    /// ```
    /// use iColor::Color;
    ///
    /// let mut color = Color::from_rgba(0,0,0,0.3).unwrap();
    /// color.opaquer(0.5);
    /// assert_eq!(color.to_rgba(), "rgba(0,0,0,0.45)");
    /// 
    /// color.opaquer(0.5);
    /// assert_eq!(color.to_rgba(), "rgba(0,0,0,0.67)");
    /// ``` 

    pub fn opaquer(&mut self, ratio: f32) -> &mut Self {
        let ratio = ratio.max(0.0).min(1.0);
        self.3 = ((self.3 + self.3 * ratio).min(1.0) * 100.0).round() / 100.0;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Color::from("#ff00aa").is_ok());
        assert!(Color::from("#f0a").is_ok());
        assert!(Color::from("#ff00aa80").is_ok());
        assert!(Color::from("rgb(129,45,78)").is_ok());
        assert!(Color::from("rgba(129,45,78, 0.8)").is_ok());
        assert!(Color::from("hsl(120, 45%, 90%)").is_ok());
        assert!(Color::from("hsla(120, 45%, 90%, 0.5)").is_ok());
        assert!(Color::from("hsv(120, 60%, 80%)").is_ok());
        assert!(Color::from("cmyk(100, 40,70,90)").is_ok());

        assert!(Color::from("#zz00aa").is_err());
        assert!(Color::from("#f0aa").is_err());
        assert!(Color::from("#ff00aaZ0").is_err());

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

        color.set_alpha(0.5);

        assert_eq!(color.to_hex(), "#FF7FD4");
        assert_eq!(color.to_rgba(), "rgba(255,0,170,0.5)");

        assert!(!color.is_dark());
        assert!(color.is_light());

        color.fade(0.5);
        assert_eq!(color.to_hex(), "#FFBFE9");

        color.opaquer(0.8);
        assert_eq!(color.to_hex(), "#FF8CD8");
        
    }
}