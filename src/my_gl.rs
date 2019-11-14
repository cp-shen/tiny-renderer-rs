//type Vec3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Vec2 = cgmath::Vector2<f32>;

use image::*;
use std::result::Result::*;

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
fn draw_tri(
    t0: Vec2,
    t1: Vec2,
    t2: Vec2,
    mut canvas: RgbaImage,
    pixel: Rgba<u8>,
) -> Result<(), ()> {
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
            let p_center = Vec2 { x: p.x as f32 + 0.5,
                y: p.y as f32 + 0.5,
            };

            let w0 = edge_fn(t1, t2, p_center);
            let w1 = edge_fn(t2, t0, p_center);
            let w2 = edge_fn(t0, t1, p_center);

            overlaps &= if w0 == 0_f32 { (edge12.y == 0_f32 && edge12.x < 0_f32) || edge12.y < 0_f32 } else { w0 > 0_f32 };
            overlaps &= if w1 == 0_f32 { (edge02.y == 0_f32 && edge02.x < 0_f32) || edge02.y < 0_f32 } else { w1 > 0_f32 };
            overlaps &= if w2 == 0_f32 { (edge01.y == 0_f32 && edge01.x < 0_f32) || edge01.y < 0_f32 } else { w2 > 0_f32 };

            if overlaps {
                canvas.put_pixel(x as u32, y as u32, pixel);
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
