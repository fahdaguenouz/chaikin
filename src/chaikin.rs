use macroquad::prelude::*;

pub fn chaikin(points: &[Vec2]) -> Vec<Vec2> {
    let mut new_points = Vec::new();
    if points.len() < 2 {
        return points.to_vec();
    }
       new_points.push(Vec2::new(points[0].x, points[0].y));
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];
        let q = p1.lerp(p2, 0.25);
        let r = p1.lerp(p2, 0.75);
        new_points.push(q);
        new_points.push(r);
    }
      new_points.push(Vec2::new(points[points.len() - 1].x, points[points.len() - 1].y));
    new_points
}
