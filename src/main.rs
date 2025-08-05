use macroquad::prelude::*;

#[macroquad::main("Chaikin")]
async fn main() {
    let mut positions: Vec<Vec2> = Vec::new();

    println!("Window created! Press ESC to exit.");
    println!("Click to add points, press Enter to draw line between last 2 points, press C to clear.");

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
                let point1 = positions[positions.len() - 2];
                let point2 = positions[positions.len() - 1];
                draw_line(point1.x, point1.y, point2.x, point2.y, 2.0, WHITE);
                println!(
                    "Drawing line from ({:.1}, {:.1}) to ({:.1}, {:.1})",
                    point1.x, point1.y, point2.x, point2.y
                );
            }
        }

        if is_key_pressed(KeyCode::C) {
            positions.clear();
            println!("Cleared all points");
        }

        if is_key_pressed(KeyCode::L) {
            if positions.len() >= 2 {
                println!(
                    "Drawing connected lines through {} points",
                    positions.len()
                );
                for i in 0..positions.len() - 1 {
                    let p1 = positions[i];
                    let p2 = positions[i + 1];
                    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, GRAY);
                }
            } else {
                println!("Need at least 2 points to draw connected lines!");
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            println!("Escape pressed, exiting...");
            break;
        }

        // Draw dots for all positions
        for pos in &positions {
            draw_circle(pos.x, pos.y, 4.0, RED);
        }

        next_frame().await;
    }
}


// Function to draw a small dot at clicked positions
fn draw_dot(
    frame: &mut [u8],
    width: u32,
    height: u32,
    (cx, cy): (f64, f64),
    radius: i32,
) {
    let center_x = cx.round() as i32;
    let center_y = cy.round() as i32;
    
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                let x = center_x + dx;
                let y = center_y + dy;
                
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    let idx = ((y as u32 * width + x as u32) * 4) as usize;
                    if idx + 3 < frame.len() {
                        frame[idx] = 255;     // R - Red dot
                        frame[idx + 1] = 0;   // G
                        frame[idx + 2] = 0;   // B
                        frame[idx + 3] = 255; // A
                    }
                }
            }
        }
    }
}

