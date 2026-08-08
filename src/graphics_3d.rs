use spin::Mutex;
use crate::screen::*;

/// Struct camera
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

/// Projection center distance from the
/// projection center
const PC_DISTANCE: f32 = 50.0;

/// Camera
static CAMERA: Mutex<Camera> = Mutex::new(Camera{
    p0: [0.0, 0.0, 0.0],
    n: [0.0, 0.0, 1.0],
    v: [0.0, 1.0, 0.0],
    u: [1.0, 0.0, 0.0],
    pc: [0.0, 0.0, PC_DISTANCE],
});

/// Convert a 3D point to the 2D projection
fn convert_to_2d(q: [f32; 3]) -> [f32; 2]{
    let camera = CAMERA.lock();
    
    let mut p: [f32;2] = [0.0;2];
    
    p[0] = q[0]*camera.pc[2]/(-q[2]+camera.pc[2]); 
    p[1] = q[1]*camera.pc[2]/(-q[2]+camera.pc[2]);
    
    return p;
}

/// Dot product between two arrays
fn dot(p: [f32;3], q: [f32;3]) -> f32{
    return p[0]*q[0] + p[1]*q[1] + p[2]*q[2];
}

/// Convert a point in the universe coordinates to the camera coordinates
fn convert_to_camera_coord(q: [f32; 3]) -> [f32; 3]{
    let camera = CAMERA.lock();
    
    let mut p: [f32;3] = [0.0;3];
    
    p[0] = camera.u[0] * q[0] + camera.u[1] * q[1] + camera.u[2] * q[2] - dot(camera.u, camera.p0);
    p[1] = camera.v[0] * q[0] + camera.v[1] * q[1] + camera.v[2] * q[2] - dot(camera.v, camera.p0);
    p[2] = camera.n[0] * q[0] + camera.n[1] * q[1] + camera.n[2] * q[2] - dot(camera.n, camera.p0);
    
    return p;
}

/// Find a line segment that is visible
fn find_segment(p: &mut [f32; 3], q: [f32; 3]) {
    // Using the 3d line equation (x-x0)/(x1-x0) = (y-y0)/(y1-y0) = (z-z0)/(z1-z0)
    
    // Calculate the value based on z
    let ratio: f32 = (PC_DISTANCE - p[2])/(q[2] - p[2]);
    
    // Find x
    p[0] = (q[0]-p[0])*ratio + p[0];
    
    // Find y
    p[1] = (q[1]-p[1])*ratio + p[1];
    
    // Set z
    p[2] = PC_DISTANCE-1.0;
}

/// Get two 3D points and convert them
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
        draw_line_window(p_2d, q_2d, [0.0, 0.0]);
    }
}

/// Rotate in the x axis
pub fn rotate_x_3d(ang: f32, p: &mut [f32;3]) {
    let z0: f32 = p[2];
    let y0: f32 = p[1];
    
    let cos: f32 = ang.cos();
    let sin: f32 = ang.sin();
    
    p[1] = y0*cos - z0*sin;
    p[2] = z0*cos + y0*sin;
}

/// Rotate in the y axis
pub fn rotate_y_3d(ang: f32, p: &mut [f32;3]) {
    let z0: f32 = p[2];
    let x0: f32 = p[0];
    
    let cos: f32 = ang.cos();
    let sin: f32 = ang.sin();
    
    p[2] = z0*cos - x0*sin;
    p[0] = x0*cos + z0*sin;
}

/// Rotate in the z axis
pub fn rotate_z_3d(ang: f32, p: &mut [f32;3]) {
    let x0: f32 = p[0];
    let y0: f32 = p[1];
    
    let cos: f32 = ang.cos();
    let sin: f32 = ang.sin();
    
    p[0] = x0*cos - y0*sin;
    p[1] = y0*cos + x0*sin;
}

/// Rotate a point in ang radians in the corresponding axis
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

/// Translate a point
pub fn translate_3d(p: &mut [f32;3], dp: [f32;3]){
    *p = [p[0]+dp[0], p[1]+dp[1], p[2]+dp[2]];
}

/// Put camera in a determined position
pub fn put_camera(new_p0: [f32;3]){
    let mut camera = CAMERA.lock();
    
    camera.p0 = new_p0;
}

/// Translate camera
pub fn translate_camera(dx: f32, dy: f32, dz: f32){
    let mut camera = CAMERA.lock();
    
    camera.p0 = [camera.p0[0]+dx, camera.p0[1]+dy, camera.p0[2]+dz];
}

/// Rotate camera
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

/// Get camera position
pub fn camera_position() -> [f32;3]{
    let camera = CAMERA.lock();
    
    return camera.p0;
}

/// Get camera N vector
pub fn camera_n() -> [f32;3]{
    let camera = CAMERA.lock();
    
    return camera.n;
}

/// Get camera V vector
pub fn camera_v() -> [f32;3]{
    let camera = CAMERA.lock();
    
    return camera.v;
}

/// Get camera U vector
pub fn camera_u() -> [f32;3]{
    let camera = CAMERA.lock();
    
    return camera.u;
}
