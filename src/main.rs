use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::dpi::PhysicalPosition;
use pixels::{Pixels, SurfaceTexture};

fn main() {
    let event_loop = EventLoop::new();
    let mut last_cursor_position: Option<PhysicalPosition<f64>> = None;
    let mut positions: Vec<(f64, f64)> = vec![];

    let window = WindowBuilder::new()
        .with_title("Chaikin")
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

    println!("Window created! Press ESC to exit.");
    println!("Click to add points, press Enter to draw line between last 2 points, press C to clear.");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("Close button clicked!");
                        *control_flow = ControlFlow::Exit;
                    }

                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(key_code),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        match key_code {
                            VirtualKeyCode::Escape => {
                                println!("Escape pressed, exiting...");
                                *control_flow = ControlFlow::Exit;
                            }
                            VirtualKeyCode::Return => {
                                if positions.len() < 2 {
                                    println!(
                                        "Need at least 2 points to draw a line! Current points: {}",
                                        positions.len()
                                    );
                                } else {
                                    pixels.frame_mut().fill(0);
                                    let point1 = positions[positions.len() - 2];
                                    let point2 = positions[positions.len() - 1];
                                    println!(
                                        "Drawing line from ({:.1}, {:.1}) to ({:.1}, {:.1})",
                                        point1.0, point1.1, point2.0, point2.1
                                    );
                                }
                            }
                            VirtualKeyCode::C => {
                                positions.clear();
                                pixels.frame_mut().fill(0);
                                window.request_redraw();
                                println!("Cleared all points");
                            }
                            VirtualKeyCode::L => {
                                if positions.len() >= 2 {
                                   
                                    println!(
                                        "Drawing connected lines through {} points",
                                        positions.len()
                                    );
                                } else {
                                    println!("Need at least 2 points to draw connected lines!");
                                }
                            }
                            _ => {
                                println!("Key pressed: {:?}", key_code);
                            }
                        }
                    }

                    WindowEvent::MouseInput { state, button, .. } => {
                        if let Some(position) = last_cursor_position {
                            if state == ElementState::Pressed && button == MouseButton::Left {
                                positions.push((position.x, position.y));
                                println!(
                                    "Point {} added at position: ({:.1}, {:.1})",
                                    positions.len(),
                                    position.x,
                                    position.y
                                );

                                draw_dot(
                                    pixels.frame_mut(),
                                    window_size.width,
                                    window_size.height,
                                    (position.x, position.y),
                                    3,
                                );
                                window.request_redraw();
                            }
                        }
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        last_cursor_position = Some(position);
                    }

                    WindowEvent::Resized(new_size) => {
                        if let Err(err) = pixels.resize_surface(new_size.width, new_size.height) {
                            eprintln!("Resize error: {}", err);
                            *control_flow = ControlFlow::Exit;
                        }
                    }

                    _ => {}
                }
            }

            Event::RedrawRequested(_) => {
                if let Err(err) = pixels.render() {
                    eprintln!("Render error: {}", err);
                    *control_flow = ControlFlow::Exit;
                }
            }

            Event::MainEventsCleared => {
                // Redraw continuously if needed
                // window.request_redraw();
            }

            _ => {}
        }
    });
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

