// vec math
use raylib::math::{Rectangle, Vector2};

pub fn in_bounds_vec2(vec: Vector2, min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> bool {
    vec.x >= min_x && vec.x <= max_x && vec.y >= min_y && vec.y <= max_y
}
// pub fn rectangle_from_vecs(start_vec: Vector2, end_vector: Vector2) -> Rectangle {
//   Rectangle::new(start_vec.x, start_vec.y, , height)
// }
