mod backends;
mod model;
mod objects;
mod render_targets;
mod transform;

mod types {
    pub type Vec3 = cgmath::Vector3<f32>;
    pub type Vec2 = cgmath::Vector2<f32>;
    pub type Quat = cgmath::Quaternion<f32>;
    pub type Mat4 = cgmath::Matrix4<f32>;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        let exe_name = String::from(std::env::current_exe().unwrap().to_str().unwrap());
        println!("usage: <{}> <path_to_obj>", exe_name);
        std::process::exit(1);
    }
}
