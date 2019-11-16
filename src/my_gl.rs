type DrawResult = Result<(), ()>;

use crate::*;
use image::*;
use std::result::Result::*;
use tobj::*;

/// check the position of point p and line ab
/// # Returns
/// p on the left of ab: > 0
/// p on the right of ab: < 0
/// p lies on line ab: 0
#[allow(dead_code)]
fn edge_fn(a: Vec2, b: Vec2, p: Vec2) -> f32 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

/// draw a triangle using given vertices and color
/// the vertices are assumed to be counter-clockwise
#[allow(dead_code)]
fn draw_tri(t0: Vec2, t1: Vec2, t2: Vec2, canvas: &mut RgbaImage, color: Rgba<u8>) -> DrawResult {
    let (t0, t1, t2) = match counter_clockwise(t0, t1, t2) {
        Ok((t0, t1, t2)) => (t0, t1, t2),
        Err(()) => return Err(()),
    };

    let min_x = t0.x.min(t1.x).min(t2.x) as u32;
    let min_y = t0.y.min(t1.y).min(t2.y) as u32;
    let max_x = t0.x.max(t1.x).min(t2.x) as u32;
    let max_y = t0.y.min(t1.y).min(t2.y) as u32;

    let edge01 = t1 - t0;
    let edge02 = t2 - t0;
    let edge12 = t2 - t1;

    type Vec2i = cgmath::Vector2<i32>;
    let p = Vec2i { x: 0, y: 0 };

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let mut overlaps = true;
            let p_center = Vec2 {
                x: p.x as f32 + 0.5,
                y: p.y as f32 + 0.5,
            };

            let w0 = edge_fn(t1, t2, p_center);
            let w1 = edge_fn(t2, t0, p_center);
            let w2 = edge_fn(t0, t1, p_center);

            overlaps &= if w0 == 0_f32 {
                (edge12.y == 0_f32 && edge12.x < 0_f32) || edge12.y < 0_f32
            } else {
                w0 > 0_f32
            };
            overlaps &= if w1 == 0_f32 {
                (edge02.y == 0_f32 && edge02.x < 0_f32) || edge02.y < 0_f32
            } else {
                w1 > 0_f32
            };
            overlaps &= if w2 == 0_f32 {
                (edge01.y == 0_f32 && edge01.x < 0_f32) || edge01.y < 0_f32
            } else {
                w2 > 0_f32
            };

            if overlaps {
                canvas.put_pixel(x as u32, y as u32, color);
            }
        }
    }

    Ok(())
}

fn counter_clockwise(t0: Vec2, t1: Vec2, t2: Vec2) -> Result<(Vec2, Vec2, Vec2), ()> {
    let val = (t1.y - t0.y) * (t2.x - t1.x) - (t1.x - t0.x) * (t2.y - t1.y);

    if val == 0_f32 {
        return Err(());
    } else if val > 0_f32 {
        return Ok((t0, t1, t2));
    }

    Ok((t0, t1, t2))
}

//TODO: remove allow unused_variables
#[allow(dead_code, unused_variables)]
fn draw_tri_world(
    t0: Vec3,
    t1: Vec3,
    t2: Vec3,
    canvas: &mut RgbaImage,
    color: Rgba<u8>,
) -> DrawResult {
    unimplemented!();
}

#[allow(dead_code)]
fn draw_mesh(mesh: Mesh, canvas: &mut RgbaImage, color: Rgba<u8>) -> DrawResult {
    assert_eq!(mesh.positions.len() % 3, 0);
    let num_vert = mesh.positions.len() / 3;

    assert_eq!(num_vert % 3, 0);
    let num_tri = num_vert / 3;

    let mut tri: [Vec3; 3];
    for i in 0..num_tri {
        tri = [
            Vec3 {
                x: mesh.positions[i * 9 + 0],
                y: mesh.positions[i * 9 + 1],
                z: mesh.positions[i * 9 + 2],
            },
            Vec3 {
                x: mesh.positions[i * 9 + 3],
                y: mesh.positions[i * 9 + 4],
                z: mesh.positions[i * 9 + 5],
            },
            Vec3 {
                x: mesh.positions[i * 9 + 6],
                y: mesh.positions[i * 9 + 7],
                z: mesh.positions[i * 9 + 8],
            },
        ];

        draw_tri_world(tri[0], tri[1], tri[2], canvas, color)?
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn draw_human_head() {
        use std::path::Path;
        let mod_load_result =
            tobj::load_obj(&Path::new("resources/models/african_head/african_head.obj"));

        assert!(mod_load_result.is_ok());

        unimplemented!();
    }
}
