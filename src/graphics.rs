use spin::Mutex;

// Struct camera declaration
struct Camera {
    // Camera position (in universe coordinates)
    p0: [f32;3],
    // Normal vector to the projection plane (in universe coordinates)
    n: [f32;3],
    //  Vector in the yview direction (in universe coordinates)
    v: [f32;3],
    //  Vector in the xview direction (in universe coordinates)
    u: [f32;3],
    // Projection center (in camera coordinates)
    pc: [f32;3],
}

// Resolution
const COLS: usize = 190;
const ROWS: usize = 44;
const SCREEN_SIZE: usize = COLS*ROWS;

const PC_DISTANCE: f32 = 50.0;

// Camera
static CAMERA: Mutex<Camera> = Mutex::new(Camera{
    p0: [0.0, 0.0, 0.0],
    n: [0.0, 0.0, 1.0],
    v: [0.0, -1.0, 0.0],
    u: [1.0, 0.0, 0.0],
    pc: [0.0, 0.0, PC_DISTANCE],
});

// Screen (in viewport coordinates)
static SCREEN: Mutex<[u8; SCREEN_SIZE]> = Mutex::new([b' '; SCREEN_SIZE]);

// Char that will be printed as a pixel
static PIXEL_CHAR: Mutex<u8> = Mutex::new(b'#');

/*
* Draw a segment on the SCREEN
*/
fn draw_segment(og: [i32; 2], dst: [i32; 2]) -> usize {
    
    // Max line size
    const MAX_SIZE: usize = 5000;
    
    // Size of the biggest possible line on the SCREEN 25x25
    let mut line_x: [i32; MAX_SIZE] = [0;MAX_SIZE];
    let mut line_y: [i32; MAX_SIZE] = [0;MAX_SIZE];
    
     // Line equation
    let mut x0: f32 = og[0] as f32;
    let mut y0: f32 = og[1] as f32;
    let mut x1: f32 = dst[0] as f32;
    let mut y1: f32 = dst[1] as f32;
    let mut x: f32 = x0;
    let mut y: f32 = y0;
    
    // Make sure ordering is preserved
    if og[0] > dst [0] {
        x0 = dst[0] as f32;
        y0 = dst[1] as f32;
        x1 = og[0] as f32;
        y1 = og[1] as f32;
        x = x0;
        y = y0;
    }
    
    // Next position in the line arrays
    let mut cursor: usize; 

    cursor = 0;
    
    // Check if it is vertical x1 == x0
    if og[0] == dst[0] {
        // Make sure ordering is preserved
        if og[1] > dst [1] {
            y0 = dst[1] as f32;
            y1 = og[1] as f32;
            y = y0;
        }

        // Vertical Line    
        while y <= y1 && cursor < MAX_SIZE {
            line_x[cursor] = x0 as i32;
            line_y[cursor] = y as i32;
            cursor += 1;
            y += 1.0;
        }
    }else{
        while x <= x1 && cursor < MAX_SIZE {
            line_x[cursor] = x as i32;
            line_y[cursor] = y as i32;
            
            // Using the line equation y = (y1 - y0)/(x1 - x0)*(x-x0)+y0
            y = (y1 - y0)/(x1 - x0)*(x - x0) + y0;
            
            cursor += 1;
            x += 0.1;
        }
    }
    
    // Aquire lock
    let mut screen = SCREEN.lock();
    
    // Draw the line in the SCREEN
    let pixel = PIXEL_CHAR.lock();
    
    for i in 0..cursor {
        // cols + line*n_cols
        screen[line_x[i] as usize + line_y[i] as usize * COLS] = *pixel;
    }
    
    return cursor;
}

// Clip the points in the window before converting to the viewport
fn clip(x0: &mut f32, y0: &mut f32, x1: &mut f32, y1: &mut f32, xmin: f32, ymin: f32, xmax: f32, ymax: f32) -> bool{
    
    // Auxiliary variables
    let mut x_aux: f32;
    let mut y_aux: f32;
    
    // Check if the line between the points is visible
    if (*x0 < xmin && *x1 < xmin) || (*x0 > xmax && *x1 > xmax) {
        return false;
    }
    
    if (*y0 < ymin && *y1 < ymin) || (*y0 > ymax && *y1 > ymax) {
        return false;
    }
    
    if *x0 < xmin || *x0 > xmax {
        if *x0 < xmin {
            x_aux = xmin;
        } else {
            x_aux = xmax;
        }

        y_aux = (*y1 - *y0) / (*x1 - *x0) * (x_aux - *x0) + *y0;

        if y_aux < ymin {
            x_aux = (*x1 - *x0) / (*y1 - *y0) * (ymin - *y0) + *x0;
            y_aux = ymin;
        } else if y_aux > ymax {
            x_aux = (*x1 - *x0) / (*y1 - *y0) * (ymax - *y0) + *x0;
            y_aux = ymax;
        }
        
        if x_aux < xmin || x_aux > xmax {
            return false;
        }

        *x0 = x_aux;
        *y0 = y_aux;     
    } else if *y0 < ymin || *y0 > ymax {
        if *y0 < ymin {
            y_aux = ymin;
        } else {
            y_aux = ymax;
        }

        x_aux = (*x1 - *x0) / (*y1 - *y0) * (y_aux - *y0) + *x0;

        if x_aux < xmin {
            y_aux = (*y1 - *y0) / (*x1 - *x0) * (xmin - *x0) + *y0;
            x_aux = xmin;
        } else if x_aux > xmax {
            y_aux = (*y1 - *y0) / (*x1 - *x0) * (xmax - *x0) + *y0;
            x_aux = xmax;
        }
        
        if y_aux < ymin || y_aux > ymax {
            return false;
        }        
        
        *x0 = x_aux;
        *y0 = y_aux;
    }

    if *x1 < xmin || *x1 > xmax {
        if *x1 < xmin {
            x_aux = xmin;
        } else {
            x_aux = xmax;
        }

        y_aux = (*y1 - *y0) / (*x1 - *x0) * (x_aux - *x0) + *y0;

        if y_aux < ymin {
            x_aux = (*x1 - *x0) / (*y1 - *y0) * (ymin - *y0) + *x0;
            y_aux = ymin;
        } else if y_aux > ymax {
            x_aux = (*x1 - *x0) / (*y1 - *y0) * (ymax - *y0) + *x0;
            y_aux = ymax;
        }
        
        if x_aux < xmin || x_aux > xmax {
            return false;
        }
        
        *x1 = x_aux;
        *y1 = y_aux;
    } else if *y1 < ymin || *y1 > ymax {
        if *y1 < ymin {
            y_aux = ymin;
        } else {
            y_aux = ymax;
        }

        x_aux = (*x1 - *x0) / (*y1 - *y0) * (y_aux - *y0) + *x0;

        if x_aux < xmin {
            y_aux = (*y1 - *y0) / (*x1 - *x0) * (xmin - *x0) + *y0;
            x_aux = xmin;
        } else if x_aux > xmax {
            y_aux = (*y1 - *y0) / (*x1 - *x0) * (xmax - *x0) + *y0;
            x_aux = xmax;
        }
        
        if y_aux < ymin || y_aux > ymax {
            return false;
        }
        
        *x1 = x_aux;
        *y1 = y_aux;
    }

    return true;
}

// Converts from the 2D Window coordinates to the viewport
fn draw_line(og: [f32; 2], dst: [f32; 2], wc: [f32; 2]){
    
    // Window limits
    let xmin: f32 = wc[0] - ((COLS as f32)/2.0 - 1.0);
    let xmax: f32 = wc[0] + (COLS as f32)/2.0 - 1.0;
    let ymin: f32 = wc[1] - ((ROWS as f32)/2.0 - 1.0);
    let ymax: f32 = wc[1] + (ROWS as f32)/2.0 - 1.0;
    
    let mut x0: f32 = og[0];
    let mut y0: f32 = og[1]/2.0; // Divide to account for the distortion of the terminal
    let mut x1: f32 = dst[0];
    let mut y1: f32 = dst[1]/2.0; // Divide to account for the distortion of the terminal
    
    // See if the line is visible after clipping
    let line_visible: bool;

    line_visible = clip(&mut x0, &mut y0, &mut x1, &mut y1, xmin, ymin, xmax, ymax);
    
    if line_visible {
        // Convert window coordinates to viewport coordinates
        x0 = (x0 - xmin)*(COLS as f32 - 1.0)/(xmax-xmin);
        y0 = (y0 - ymin)*(ROWS as f32 - 1.0)/(ymax-ymin);
        
        x1 = (x1 - xmin)*(COLS as f32 - 1.0)/(xmax-xmin);
        y1 = (y1 - ymin)*(ROWS as f32 - 1.0)/(ymax-ymin);
        
        // Draw on SCREEN
        draw_segment([x0 as i32, y0 as i32], [x1 as i32, y1 as i32]);
    }
}

/*
* Convert a 3D point to the 2D projection
*/
fn convert_to_2d(q: [f32; 3]) -> [f32; 2]{
    let camera = CAMERA.lock();
    
    let mut p: [f32;2] = [0.0;2];
    
    p[0] = q[0]*camera.pc[2]/(-q[2]+camera.pc[2]); 
    p[1] = q[1]*camera.pc[2]/(-q[2]+camera.pc[2]);
    
    return p;
}

// Dot product between two arrays
fn dot(p: [f32;3], q: [f32;3]) -> f32{
    return p[0]*q[0] + p[1]*q[1] + p[2]*q[2];
}

/*
* Convert a point in the universe coordinates to the camera coordinates
*/
fn convert_to_camera_coord(q: [f32; 3]) -> [f32; 3]{
    let camera = CAMERA.lock();
    
    let mut p: [f32;3] = [0.0;3];
    
    p[0] = camera.u[0] * q[0] + camera.u[1] * q[1] + camera.u[2] * q[2] - dot(camera.u, camera.p0);
    p[1] = camera.v[0] * q[0] + camera.v[1] * q[1] + camera.v[2] * q[2] - dot(camera.v, camera.p0);
    p[2] = camera.n[0] * q[0] + camera.n[1] * q[1] + camera.n[2] * q[2] - dot(camera.n, camera.p0);
    
    return p;
}

// Find a line segment that is visible
fn find_segment(p: &mut [f32; 3], q: [f32; 3]) {
    // Using the 3d line equation (x-x0)/(x1-x0) = (y-y0)/(y1-y0) = (z-z0)/(z1-z0)
    
    // Calculate the value based on z
    let ratio: f32 = (49.0 - p[2])/(q[2] - p[2]);
    
    // Find x
    p[0] = (q[0]-p[0])*ratio + p[0];
    
    // Find y
    p[1] = (q[1]-p[1])*ratio + p[1];
    
    // Set z
    p[2] = PC_DISTANCE-1.0;
}

/*
* Get two 3D points and convert them
*/
pub fn line_3d(og: [f32; 3], dst: [f32; 3]) {    
    // Convert points from universe to camera coordinates
    let mut p_3d: [f32; 3] = convert_to_camera_coord(og);
    let mut q_3d: [f32; 3] = convert_to_camera_coord(dst);
    
    let p_2d: [f32; 2];
    let q_2d: [f32; 2];
    
    // Try to find a visible segment
    if p_3d[2] < PC_DISTANCE && q_3d[2] >= PC_DISTANCE {
        find_segment(&mut q_3d, p_3d);
    } else if p_3d[2] >= PC_DISTANCE && q_3d[2] < PC_DISTANCE {
        find_segment(&mut p_3d, q_3d);
    }
    
    // Check if the points are in the cameras field of view
    if p_3d[2] < PC_DISTANCE && q_3d[2] < PC_DISTANCE {
        // Convert points to 2D projection
        p_2d = convert_to_2d(p_3d);
        q_2d = convert_to_2d(q_3d);
        // Prepare the window to draw with center in 0, 0
        draw_line(p_2d, q_2d, [0.0, 0.0]);
    }
}

// Rotate in the x axis
pub fn rotate_x_3d(ang: f32, p: &mut [f32;3]) {
    let z0: f32 = p[2];
    let y0: f32 = p[1];
    
    let cos: f32 = ang.cos();
    let sin: f32 = ang.sin();
    
    p[1] = y0*cos - z0*sin;
    p[2] = z0*cos + y0*sin;
}

// Rotate in the y axis
pub fn rotate_y_3d(ang: f32, p: &mut [f32;3]) {
    let z0: f32 = p[2];
    let x0: f32 = p[0];
    
    let cos: f32 = ang.cos();
    let sin: f32 = ang.sin();
    
    p[2] = z0*cos - x0*sin;
    p[0] = x0*cos + z0*sin;
}

// Rotate in the z axis
pub fn rotate_z_3d(ang: f32, p: &mut [f32;3]) {
    let x0: f32 = p[0];
    let y0: f32 = p[1];
    
    let cos: f32 = ang.cos();
    let sin: f32 = ang.sin();
    
    p[0] = x0*cos - y0*sin;
    p[1] = y0*cos + x0*sin;
}

// Rotate a point in ang radians in the corresponding axis
pub fn rotate_3d(ang: f32, p: &mut [f32;3], axis: [f32;3]){
    let mut ref_axis: [f32;3] = [axis[0], axis[1], axis[2]];
    
    // Angle of rotation in the zx plane
    let angzx: f32;
    
    // Angle of rotation in the yz plane
    let angyz: f32;
       
    // Get the angle between the axis and the yz plane
    angzx = axis[0].atan2(axis[2]);        

    if angzx != 0.0 {
        rotate_y_3d(-angzx, p);
        rotate_y_3d(-angzx, &mut ref_axis); // rotate the axis to get angyz later
    }
    
    // Get the angle between the axis and y axis when it is in the yz plane
    angyz = ref_axis[2].atan2(ref_axis[1]);
    if angyz != 0.0 {
        rotate_x_3d(-angyz, p);
    }
     
    // Apply the proper rotation
    rotate_y_3d(ang, p);
    
    // Undo the first rotations if needed
    if angyz != 0.0 {
        rotate_x_3d(angyz, p);
    }
    
    if angzx != 0.0 {
        rotate_y_3d(angzx, p);
    }
}

// Translate a point
pub fn translate_3d(p: &mut [f32;3], dp: [f32;3]){
    *p = [p[0]+dp[0], p[1]+dp[1], p[2]+dp[2]];
}

// Put camera in a determined position
pub fn put_camera(new_p0: [f32;3]){
    let mut camera = CAMERA.lock();
    
    camera.p0 = new_p0;
}

// Translate camera
pub fn translate_camera(dx: f32, dy: f32, dz: f32){
    let mut camera = CAMERA.lock();
    
    camera.p0 = [camera.p0[0]+dx, camera.p0[1]+dy, camera.p0[2]+dz];
}

pub fn rotate_camera(angh: f32, angv: f32){
    let mut camera = CAMERA.lock();
    let mut axis: [f32;3];
    
    if angh != 0.0 {
        axis = [0.0, 1.0, 0.0];
        rotate_3d(angh, &mut camera.v, axis);
        rotate_3d(angh, &mut camera.u, axis);
        rotate_3d(angh, &mut camera.n, axis);
    }
    
    if angv != 0.0 {
        axis = [camera.u[0], camera.u[1], camera.u[2]];
        
        // Apply proper rotation
        rotate_3d(angv, &mut camera.v, axis);
        rotate_3d(angv, &mut camera.n, axis);
        rotate_3d(angv, &mut camera.u, axis);
    }
}

pub fn pixel_char(chr: u8){
    let mut pixel = PIXEL_CHAR.lock();
    
    *pixel = chr;
}

// Get camera position
pub fn camera_position() -> [f32;3]{
    let camera = CAMERA.lock();
    
    return camera.p0;
}

// Get camera N vector
pub fn camera_n() -> [f32;3]{
    let camera = CAMERA.lock();
    
    return camera.n;
}

// Get camera V vector
pub fn camera_v() -> [f32;3]{
    let camera = CAMERA.lock();
    
    return camera.v;
}

// Get camera U vector
pub fn camera_u() -> [f32;3]{
    let camera = CAMERA.lock();
    
    return camera.u;
}

pub fn clear_screen() {
    // Aquire lock
    let mut screen = SCREEN.lock();
    *screen = [b' '; SCREEN_SIZE];
    
    // Go back to the first position (0, 0)
    print!("\x1B[H");
}

pub fn print_screen() {
    // Aquire lock
    let screen = SCREEN.lock();

    for i in 0..ROWS {
        for j in 0..COLS {
            // Print pixel
            print!("{}", screen[j + i*COLS] as char);
        }
        // Next line
        println!();
        
        // Carriage return
        print!("\x1B[G");
    }
}
