use macroquad::prelude::*;

#[macroquad::main("Chaikin")]
async fn main() {
    let mut positions: Vec<Vec2> = Vec::new();
    let mut lines: Vec<(Vec2, Vec2)> = Vec::new();

    println!("Window created! Press ESC to exit.");
    println!("Click to add points, press Enter to draw persistent lines connecting all points in a loop, press C to clear, press L for open polyline.");

    loop {
        clear_background(BLACK);

        // Handle input
        if is_mouse_button_pressed(MouseButton::Left) {
            let pos = mouse_position().into();
            positions.push(pos);
            println!(
                "Point {} added at position: ({:.1}, {:.1})",
                positions.len(),
                pos.x,
                pos.y
            );
        }

        if is_key_pressed(KeyCode::Enter) {
            if positions.len() < 2 {
                println!(
                    "Need at least 2 points to draw a line! Current points: {}",
                    positions.len()
                );
            } else {
                println!(
                    "Drawing closed loop through {} points",
                    positions.len()
                );
                // Clear previous lines
                lines.clear();
                // Store lines between consecutive points
                for i in 0..positions.len() - 1 {
                    let p1 = positions[i];
                    let p2 = positions[i + 1];
                    lines.push((p1, p2));
                    println!(
                        "Drawing line from ({:.1}, {:.1}) to ({:.1}, {:.1})",
                        p1.x, p1.y, p2.x, p2.y
                    );
                }
                // Close the loop by connecting the last point to the first
                if positions.len() >= 2 {
                    let p1 = positions[positions.len() - 1];
                    let p2 = positions[0];
                    lines.push((p1, p2));
                    println!(
                        "Drawing closing line from ({:.1}, {:.1}) to ({:.1}, {:.1})",
                        p1.x, p1.y, p2.x, p2.y
                    );
                }
            }
        }

        if is_key_pressed(KeyCode::C) {
            positions.clear();
            lines.clear();
            println!("Cleared all points and lines");
        }

        if is_key_pressed(KeyCode::L) {
            if positions.len() >= 2 {
                println!(
                    "Drawing open polyline through {} points",
                    positions.len()
                );
                for i in 0..positions.len() - 1 {
                    let p1 = positions[i];
                    let p2 = positions[i + 1];
                    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, GRAY);
                    println!(
                        "Drawing line from ({:.1}, {:.1}) to ({:.1}, {:.1})",
                        p1.x, p1.y, p2.x, p2.y
                    );
                }
            } else {
                println!("Need at least 2 points to draw connected lines!");
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            println!("Escape pressed, exiting...");
            break;
        }


        for (p1, p2) in &lines {
            draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, WHITE);
        }


        for pos in &positions {
            draw_circle_lines(pos.x, pos.y, 2.0, 1.0, GRAY);
        }

        next_frame().await;
    }
}