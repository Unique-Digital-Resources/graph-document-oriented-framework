/// Converts a Hex string (e.g., "#FF0000" or "#FF0000FF") to an HSLA array [H, S, L, A].
pub fn hex_to_hsla(hex: &str) -> Option<[f64; 4]> {
    let hex = hex.trim_start_matches('#');
    let (r, g, b, a) = if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
        (r, g, b, 1.0)
    } else if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        (r, g, b, 1.0)
    } else if hex.len() == 8 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
        (r, g, b, a as f64 / 255.0)
    } else {
        return None;
    };

    let r_norm = r as f64 / 255.0;
    let g_norm = g as f64 / 255.0;
    let b_norm = b as f64 / 255.0;

    let max = r_norm.max(g_norm).max(b_norm);
    let min = r_norm.min(g_norm).min(b_norm);
    let l = (max + min) / 2.0;

    let (h, s) = if max == min {
        (0.0, 0.0) // Achromatic
    } else {
        let d = max - min;
        let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
        let h = if max == r_norm {
            (g_norm - b_norm) / d + (if g_norm < b_norm { 4.0 } else { 0.0 })
        } else if max == g_norm {
            (b_norm - r_norm) / d + 2.0
        } else {
            (r_norm - g_norm) / d + 4.0
        };
        (h * 60.0, s)
    };

    Some([h, s * 100.0, l * 100.0, a])
}

/// Converts an HSLA array [H, S, L, A] to a Hex string.
pub fn hsla_to_hex(hsla: &[f64]) -> String {
    let h = hsla[0] / 360.0;
    let s = hsla[1] / 100.0;
    let l = hsla[2] / 100.0;
    let a = hsla[3];

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0/6.0 { (c, x, 0.0) }
        else if h < 2.0/6.0 { (x, c, 0.0) }
        else if h < 3.0/6.0 { (0.0, c, x) }
        else if h < 4.0/6.0 { (0.0, x, c) }
        else if h < 5.0/6.0 { (x, 0.0, c) }
        else { (c, 0.0, x) };

    let to_hex = |v: f64| {
        let val = ((v + m) * 255.0).round() as u8;
        format!("{:02X}", val)
    };

    let mut hex = format!("#{}{}{}", to_hex(r), to_hex(g), to_hex(b));
    if a < 1.0 {
        let alpha = (a * 255.0).round() as u8;
        hex.push_str(&format!("{:02X}", alpha));
    }
    hex
}

/// Calculates a contrasting text color (black or white) for a given HSLA background.
pub fn get_contrast_color(hsla: &[f64]) -> String {
    // Simple luminance check
    let l = hsla[2] / 100.0;
    if l > 0.55 {
        "rgba(0,0,0,0.8)".to_string()
    } else {
        "rgba(255,255,255,0.9)".to_string()
    }
}