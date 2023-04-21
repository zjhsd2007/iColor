use regex::Match;

pub fn match_to_num(m: Option<&Match>) -> Option<u8> {
    m.map(|m| m.as_str()).and_then(|s| {
        s.repeat(2)
            .get(0..2)
            .and_then(|s| u8::from_str_radix(s, 16).ok())
    })
}

pub fn match_to_num2(m: Option<&Match>) -> Option<u8> {
    m.map(|m| m.as_str()).and_then(|s| s.parse::<u8>().ok())
}

pub fn calc_rgb_with_alpha(v: u8, alpha: f32) -> f32 {
    v as f32 * alpha + 255.0 * (1.0 - alpha)
}

pub fn is_valid_num(v: &f32) -> bool {
    (0.0..=1.0).contains(v)
}
