use macroquad::prelude::*;


    pub fn chaikin(points: &[Vec2], is_closed: bool) -> Vec<Vec2> {
        let mut new_points = Vec::new();
        if points.len() < 2 {
            return points.to_vec();
        }
        for i in 0..points.len() - 1 {
            let p1 = points[i];
            let p2 = points[i + 1];
            // Compute Q = 0.75*p1 + 0.25*p2
            let q = Vec2::new(p1.x * 0.75 + p2.x * 0.25, p1.y * 0.75 + p2.y * 0.25);
            // Compute R = 0.25*p1 + 0.75*p2
            let r = Vec2::new(p1.x * 0.25 + p2.x * 0.75, p1.y * 0.25 + p2.y * 0.75);
            new_points.push(q);
            new_points.push(r);
        }
        if is_closed && points.len() >= 2 {
            let p1 = points[points.len() - 1];
            let p2 = points[0];
            let q = Vec2::new(p1.x * 0.75 + p2.x * 0.25, p1.y * 0.75 + p2.y * 0.25);
            let r = Vec2::new(p1.x * 0.25 + p2.x * 0.75, p1.y * 0.25 + p2.y * 0.75);
            new_points.push(q);
            new_points.push(r);
        }
        new_points
    }
