use spin::Mutex;
use crate::screen::{draw_point_window,
                    draw_line_window,
                    pixel_char,
                    current_pixel_char};

/// Window center is the center of what is being
/// shown on the screen. If something is outside
/// the bounds, it will not render
static WIN_CENTER: Mutex<[f32; 2]> = Mutex::new([0.0; 2]);

/// Draw a 2d point
pub fn point_2d(p: [f32; 2]) {
    let wc = WIN_CENTER.lock();
    draw_point_window(p, *wc);
}

/// Draw a 2d line
pub fn line_2d(og: [f32; 2], dst: [f32; 2]) {
    let wc = WIN_CENTER.lock();
    draw_line_window(og, dst, *wc);
}

/// Text label
pub fn label_2d(s: &str, p: [f32; 2]) {
    let mut msg: &str = s;
    let mut i: f32 = 0.0;
    let old_char: u8 = current_pixel_char();
    
    // Check if string slice is ASCII
    if !s.is_ascii() {
      msg = "ERROR: string not an ASCII.";
    }
    
    // Print message
    for byte in msg.bytes() {
        pixel_char(byte);
        point_2d([p[0]+i, p[1]]);
        i += 1.0;
    }
    // Restore old character
    pixel_char(old_char);
}

/// Rotate (in the z axis)
pub fn rotate_2d(p: &mut [f32;2], ang: f32) {
    let x0: f32 = p[0];
    let y0: f32 = p[1];
    
    let cos: f32 = ang.cos();
    let sin: f32 = ang.sin();
    
    p[0] = x0*cos - y0*sin;
    p[1] = y0*cos + x0*sin;
}

/// Translate a point
pub fn translate_2d(p: &mut [f32;2], dp: [f32;2]){
    *p = [p[0]+dp[0], p[1]+dp[1]];
}

/// Move the window center to win
pub fn put_window(win: [f32; 2]) {
    let mut wc = WIN_CENTER.lock();
    *wc = win;
}

/// Translate window center based on dx and dy
pub fn translate_window(dwc: [f32; 2]) {
    let mut wc = WIN_CENTER.lock();
    wc[0] += dwc[0];
    wc[1] += dwc[1];
}
