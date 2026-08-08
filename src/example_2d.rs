use crate::graphics_2d::{line_2d, translate_window, put_window,
                        point_2d, translate_2d};

use crate::screen::{print_screen, clear_screen, pixel_char};

use std::{thread, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};

pub fn square(dp: [f32; 2]) {
  let mut p0: [f32;2] = [-5.0, 5.0];
  let mut p1: [f32;2] = [5.0, 5.0];
  let mut p2: [f32;2] = [5.0, -5.0];
  let mut p3: [f32;2] = [-5.0, -5.0];
  
  // Translate
  translate_2d(&mut p0, dp);
  translate_2d(&mut p1, dp);
  translate_2d(&mut p2, dp);
  translate_2d(&mut p3, dp);
  
  // Lines
  line_2d(p0, p1);
  line_2d(p1, p2);
  line_2d(p2, p3);
  line_2d(p0, p3);
}

fn circle(pos: [f32;2], radius: f32) {
    let mut ang1: f32 = 0.0174532925;
    const FULL_ANG: f32 = 3.14159265*2.0;
    
    let mut i: usize = 0;
    
    // Array of points
    let mut points: [[f32;2]; 360] = [[0.0;2]; 360];
    
    // Generate points
    while i < 360 && ang1 < FULL_ANG {
        points[i] = [radius*ang1.cos(), radius*ang1.sin()];
        
        // 1 degree in radians
        ang1 += 0.0174532925;
        
        i += 1;
    }
    
    // Transform points
    for j in 0..360 {
        translate_2d(&mut points[j], pos);
    }
    
    // Draw points
    for j in 0..360 {
        point_2d(points[j]);
    }
}

pub fn main_loop(){
    // Disable echo and enter to the terminal (canonical mode)
    enable_raw_mode().unwrap();
    
    // Square positions
    let s0: [f32;2] = [0.0, 0.0];
    let s1: [f32;2] = [50.0,2.0];
    
    // Moving point
    let mut p: [f32;2] = [0.0, 0.0];
    
    // Clear screen
    print!("\x1B[2J\x1B[H");
    loop {
        // Squares
        pixel_char(b'#');
        square(s0);
        square(s1);
        
        // Character
        pixel_char(b'$');
        point_2d(p);
        circle(p, 4.0);
        
        thread::sleep(Duration::from_millis(17));
        
        // Print screen to the teminal
        print_screen();
        
        // Clear screen
        clear_screen();
        
        // Check if there is an event
        if poll(Duration::from_millis(0)).unwrap() {
            // Check if it is a key event
            match read().unwrap() {
                Event::Key(event) => {
                    match event.code {
                        // Move character
                        KeyCode::Char('w') => p[1] += 1.0,
                        KeyCode::Char('s') => p[1] -= 1.0,
                        KeyCode::Char('a') => p[0] -= 1.0,
                        KeyCode::Char('d') => p[0] += 1.0,
                        
                        // Move camera
                        KeyCode::Up => translate_window([0.0,1.0]),
                        KeyCode::Down => translate_window([0.0,-1.0]),
                        KeyCode::Left => translate_window([-1.0,0.0]),
                        KeyCode::Right => translate_window([1.0,0.0]),
                        
                        KeyCode::Char('q') => break,
                        _ => ()
                    }
                },
                _ => ()
            }
        }
    }
    
    // Clear screen
    print!("\x1B[2J\x1B[H");
    disable_raw_mode().unwrap();
}
