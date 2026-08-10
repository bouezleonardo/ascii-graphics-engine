mod graphics_3d;
mod graphics_2d;
mod screen;
mod example_3d;
mod example_2d;

/// Main function
fn main() {
    // 2D example
    example_2d::main_loop();
    // 3D example
    example_3d::main_loop();
}
