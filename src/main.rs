mod graphics;

use graphics::{line_3d, print_screen, clear_screen, translate_camera, translate_3d, rotate_3d, rotate_camera, pixel_char, camera_n, camera_u};

use std::{thread, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};

fn cube(pos: [f32;3], size: f32, ang: f32, center:[f32;3], axis:[f32;3]){
    let mut a = [-size, -size, -size];
    let mut b = [ size, -size, -size];
    let mut c = [ size,  size, -size];
    let mut d = [-size,  size, -size];

    let mut e = [-size, -size,  size];
    let mut f = [ size, -size,  size];
    let mut g = [ size,  size,  size];
    let mut h = [-size,  size,  size];

    // List of references to apply the transformations
    let points = [
        &mut a, &mut b, &mut c, &mut d,
        &mut e, &mut f, &mut g, &mut h
    ];
    
    // Apply operation to all points
    for p in points {
        // Translate to the position selected
        translate_3d(p, pos);
        
        if ang != 0.0{
            // Translate back to the origin
            translate_3d(p, [-center[0], -center[1], -center[2]]);

            // Rotate
            rotate_3d(ang, p, axis);

            // Translate back to the position selected
            translate_3d(p, center);
        }
    }

    // Front
    line_3d(a,b);
    line_3d(b,c);
    line_3d(c,d);
    line_3d(d,a);

    // Back
    line_3d(e,f);
    line_3d(f,g);
    line_3d(g,h);
    line_3d(h,e);

    // Conections
    line_3d(a,e);
    line_3d(b,f);
    line_3d(c,g);
    line_3d(d,h);
}

fn circle(pos: [f32;3], radius: f32, ang: f32, center:[f32;3], axis:[f32;3]) {
    let mut ang1: f32 = 0.0174532925;
    const FULL_ANG: f32 = 3.14159265*2.0;
    
    let mut i: usize = 0;
    
    // Array of point references
    let mut points: [[f32;3]; 360] = [[0.0;3]; 360];
    
    // Generate points
    while i < 360 && ang1 < FULL_ANG {
        points[i] = [radius*ang1.cos(), radius*ang1.sin(), 0.0];
        
        ang1 += 0.0174532925;
        
        i += 1;
    }
    
    // Transform points
    for j in 0..360 {
        translate_3d(&mut points[j], pos);
        
        if ang > 0.0 {
            translate_3d(&mut points[j], [-center[0], -center[1], -center[2]]);
            
            rotate_3d(ang, &mut points[j], axis);
            
            translate_3d(&mut points[j], center);
        }
    }
    
    // Draw points
    for j in 0..359 {
        line_3d(points[j], points[j+1]);
    }
}

fn floor(){
    let size: f32 = 500.0;
    
    let mut i: f32 = -size;
    while i <= size {
        line_3d([i, -100.0, size], [i, -100.0, -size]);
        line_3d([size, -100.0, i], [-size, -100.0, i]);
        
        i += 50.0;
    } 
}

fn ceiling(){
    let size: f32 = 500.0;
    
    let mut i: f32 = -size;
    while i <= size {
        line_3d([i, 400.0, size], [i, 400.0, -size]);
        line_3d([size, 400.0, i], [-size, 400.0, i]);
        
        i += 50.0;
    } 
} 

fn foward(step: f32) {
    let n:[f32;3] = camera_n();
    
    translate_camera(step*n[0], step*n[1], step*n[2]);
}

fn side(step: f32) {
    let u:[f32;3] = camera_u();
    
    translate_camera(step*u[0], step*u[1], step*u[2]);
}

fn main(){
    // Disable echo and enter to the terminal (canonical mode)
    enable_raw_mode().unwrap();
    
    let mut ang: f32 = 0.0;
    
    let mut camera_ang: f32 = 0.0;
    
    let p0: [f32;3] = [0.0, 0.0, 0.0];
    let p1: [f32;3] = [0.0,200.0, -50.0];
    let p2: [f32;3] = [150.0,150.0,150.0];
    let p3: [f32;3] = [-150.0,-150.0,150.0];
    let p4: [f32;3] = [0.0,0.0,-150.0];
    let p5: [f32;3] = [-150.0, 150.0,-150.0];
    let p6: [f32;3] = [150.0,-150.0,-150.0];
    
    // Clear screen
    print!("\x1B[2J\x1B[H");
    loop {
        pixel_char(b'+');
        floor();
        ceiling();
        
        pixel_char(b'#');
        cube(p1, 40.0, ang, p1, [0.0, 1.0, 0.0]);
        cube(p2, 40.0, ang, p2, [0.0, 0.0, 1.0]);
        cube(p3, 40.0, ang, p3, [1.0, 0.0, 0.0]);
        
        cube(p4, 40.0, ang, p4, [1.0, 1.0, 1.0]);
        cube(p5, 40.0, ang, p5, [0.0, 1.0, 1.0]);
        cube(p6, 40.0, ang, p6, [1.0, 0.0, 1.0]);
    
        
        pixel_char(b'*');
        cube([0.0,0.0,-100.0], 20.0, ang*10.0, [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        cube([0.0,0.0,-100.0], 20.0, ang*10.0, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        
        cube([5.0,0.0,-1000.0], 60.0, ang*10.0, [5.0,0.0,-1000.0], [0.0,0.0,1.0]);
        
        pixel_char(b'O');
        circle(p0, 30.0, ang*10.0, p0, [0.0, 1.0, 0.0]);
        circle(p0, 30.0, ang*10.0 + 1.570796, p0, [0.0, 1.0, 0.0]);
        circle(p0, 30.0, ang*10.0 - 0.785398, p0, [0.0, 1.0, 0.0]);
        circle(p0, 30.0, ang*10.0 + 0.785398, p0, [0.0, 1.0, 0.0]);

        // Print screen to the teminal
        print_screen();
        
        // Clear screen
        clear_screen();
        
        ang+= 0.0017;
        
        // Check if there is an event
        if poll(Duration::from_millis(0)).unwrap() {
            // Check if it is a key event
            match read().unwrap() {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('w') => foward(-4.0),
                        KeyCode::Char('s') => foward(4.0),
                        KeyCode::Char('a') => side(-4.0),
                        KeyCode::Char('d') => side(4.0),
                        
                        KeyCode::Up => {
                            // If it is less than 90 degrees
                            if camera_ang < 1.0{
                                rotate_camera(0.0, 0.2);
                                camera_ang += 0.2;
                            }
                        },
                        KeyCode::Down => {
                            // If it is less than 90 degrees
                            if camera_ang > -1.0{
                                rotate_camera(0.0, -0.2);
                                camera_ang -= 0.2;
                            }
                        },
                        KeyCode::Left => rotate_camera(0.1, 0.0),
                        KeyCode::Right => rotate_camera(-0.1, 0.0),
                        
                        KeyCode::Char('q') => break,
                        _ => ()
                    }
                },
                _ => ()
            }
        }
        thread::sleep(Duration::from_millis(17));
    }
    
    // Clear screen
    print!("\x1B[2J\x1B[H");
    disable_raw_mode().unwrap();
}
