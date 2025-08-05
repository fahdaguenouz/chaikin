pub fn draw_dot(
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
