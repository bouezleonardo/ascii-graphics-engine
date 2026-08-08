use spin::Mutex;
use crate::screen::{draw_point_window,
                    draw_line_window};

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
