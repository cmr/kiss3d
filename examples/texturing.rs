extern crate kiss3d;
extern crate "nalgebra" as na;

use na::{Vec3, Rotation};
use kiss3d::window::Window;
use kiss3d::light;

fn main() {
    let mut window = Window::new("Kiss3d: texturing");
    let mut c      = window.add_cube(1.0, 1.0, 1.0);

    c.set_color(1.0, 0.0, 0.0);
    c.set_texture_from_file(&Path::new("media/kitten.png"), "kitten");

    window.set_light(light::StickToCamera);

    while window.render() {
        c.append_rotation(&Vec3::new(0.0f32, 0.014, 0.0));
    }
}
