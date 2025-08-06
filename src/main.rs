use macroquad::prelude::*;
pub mod chaikin;
use chaikin::*;

#[macroquad::main("Chaikin")]
async fn main() {
    let mut positions: Vec<Vec2> = Vec::new(); // Control points
    let mut lines: Vec<(Vec2, Vec2)> = Vec::new(); // Persistent lines for two-point case
    let mut animation_points: Vec<Vec<Vec2>> = Vec::new(); // Points for each animation step
    let mut is_animating = false;
    let mut current_step = 0;
    let max_steps = 6;
    let mut step_timer = 0.0;
    let step_duration = 1.0; // Seconds per step

    println!("Window created! Press ESC to exit.");
    println!(
        "Click to add points, press Enter to start Chaikin animation (3+ points) or draw line (2 points), press C to clear, press L for open polyline."
    );

    loop {
        clear_background(BLACK);
        if positions.len() == 0 {
            draw_text("Click to add points", 20.0, 20.0, 20.0, WHITE);
            draw_text(
                "2 points will draw a straight line; 3 or more will create a smooth curve.",
                20.0,
                45.0,
                20.0,
                WHITE
            );
            draw_text(
                "Press C to clear, ESC to exit and Enter to start animation",
                20.0,
                70.0,
                20.0,
                WHITE
            );
        }

        if !is_animating && is_mouse_button_pressed(MouseButton::Left) {
            let pos = mouse_position().into();
            positions.push(pos);
            println!("Point {} added at position: ({:.1}, {:.1})", positions.len(), pos.x, pos.y);
        }

        if is_key_pressed(KeyCode::Enter) {
            match positions.len() {
                0 => println!("No points to process! Please add points."),
                1 => println!("Only one point, displaying point only."),
                2 => {
                    lines.clear();
                    let p1 = positions[0];
                    let p2 = positions[1];
                    lines.push((p1, p2));
                    println!(
                        "Drawing persistent line from ({:.1}, {:.1}) to ({:.1}, {:.1})",
                        p1.x,
                        p1.y,
                        p2.x,
                        p2.y
                    );
                }
                _ => {
                    is_animating = true;
                    current_step = 0;
                    step_timer = 0.0;
                    animation_points.clear();
                    lines.clear();
                    animation_points.push(positions.clone());
                    let mut current = positions.clone();
                    for _ in 0..max_steps {
                        current = chaikin(&current, false); // Open polyline
                        animation_points.push(current.clone());
                    }
                    println!(
                        "Starting Chaikin animation with {} points (open polyline)",
                        positions.len()
                    );
                }
            }
        }

        if is_key_pressed(KeyCode::C) {
            positions.clear();
            lines.clear();
            animation_points.clear();
            is_animating = false;
            current_step = 0;
            step_timer = 0.0;
            println!("Cleared all points, lines, and animation");
        }

        if is_key_pressed(KeyCode::Escape) {
            println!("Escape pressed, exiting...");
            break;
        }

        if is_animating {
            step_timer += get_frame_time();
            if step_timer >= step_duration {
                step_timer = 0.0;
                current_step = (current_step + 1) % (max_steps + 1);
                if current_step == 0 {
                    println!("Animation restarted");
                }
            }
        }

        for (p1, p2) in &lines {
            draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, WHITE);
        }

        if is_animating && !animation_points.is_empty() {
            let points = &animation_points[current_step];
            if points.len() >= 2 {
                for i in 0..points.len() - 1 {
                    let p1 = points[i];
                    let p2 = points[i + 1];
                    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, WHITE);
                }
            }
        }

        for pos in &positions {
            draw_circle_lines(pos.x, pos.y, 2.0, 1.0, GRAY);
        }

        next_frame().await;
    }
}
