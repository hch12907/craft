#![feature(const_generics)]
#![feature(maybe_uninit_ref)]

use std::time::Instant;

mod camera;
mod input;
mod maths;
mod mesh;
mod renderer;
mod windowing;
mod world;
mod utils;

use camera::*;
use input::*;
use windowing::*;

fn main() {
    let w = windowing::Window::create_window();
    let mut r = renderer::GlRenderer::new(&w, 
        maths::Matrix4::perspective(maths::Deg(55.0), 16.0/9.0, 0.1, 200.0)
    );

    let (tx, rx) = std::sync::mpsc::channel::<(i32, i32, mesh::Mesh)>();
    let (bound0, bound1) = (-32, 32);

    let world_minister = std::thread::spawn(move || {
        let tx = tx;
        
        for x in bound0..bound1 {
            for y in bound0..bound1 {
                let tx = tx.clone();
                let mut noise = world::Noise2D::<world::SineNoise>::new(
                    ((x << 5) ^ (y + 1234)) as u64
                );

                std::thread::spawn(move || {
                    let pos = maths::Vector2I::new(x, y);
                    let chunk = world::Chunk::new(pos, &mut noise);
                    let mesh = chunk.generate_mesh::<world::GreedyCubeMesher>();
                    tx.send((pos.x(), pos.y(), mesh))
                });
            }
        }

        drop(tx);
    });
    
    let speed = 10.0;

    let mut mouse_locked = false;
    let mut pos = maths::Vector3F::new(0.0, 4.5, -3.0);

    let mut cam = Camera::new(pos, maths::Vector3F::new(0.0, 0.0, 0.0));
    let mut input_manager = InputManager::new();

    let mut last_time = Instant::now();
    let mut delta = 0.0;
    let mut time = 0.0;

    w.run(move |event, cl, context| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        *cl = ControlFlow::Exit;
                    },
                    
                    WindowEvent::MouseInput { button, .. } => {
                        use glutin::event::MouseButton;
                        match button {
                            MouseButton::Left => {
                                context.window().set_cursor_grab(true).unwrap();
                                context.window().set_cursor_visible(false);
                                mouse_locked = true;
                                input_manager.unsuspend_input();
                            },
                            _ => {}
                        }
                    },

                    _ => {}
                }
            },

            Event::MainEventsCleared => {
                let mut new_speed = speed;
                let sensitivity = cam.sensitivity();
                let up = maths::Vector3F::new(0.0, 1.0, 0.0);

                cam.move_camera(pos);

                if let Ok((x, y, mesh)) = rx.recv() {
                    println!(
                        "chunk at ({}, {}) has {} vertices and {} indices",
                        x, y,
                        mesh.vertices().len(),
                        mesh.indices().len(),
                    );
                    r.render_mesh(mesh);
                }

                // Prioritise modifiers like LShift.
                for key in input_manager.iterate_held_keys() {
                    match key {
                        &Key::LShift => new_speed *= 2.0,
                        &Key::LControl => new_speed *= 0.2,
                        _ => {}
                    }
                }

                for key in input_manager.iterate_held_keys() {
                    match key {
                        &Key::W => pos += new_speed * delta * cam.front(),
                        &Key::S => pos -= new_speed * delta * cam.front(),
                        //&Key::W => pos += maths::Matrix3::rotate_y_axis(maths::Deg(-90.0)) * (new_speed * delta * cam.front().cross(up).normalize()),
                        //&Key::S => pos -= maths::Matrix3::rotate_y_axis(maths::Deg(-90.0)) * (new_speed * delta * cam.front().cross(up).normalize()),
                        &Key::A => pos -= new_speed * delta * cam.front().cross(up).normalize(),
                        &Key::D => pos += new_speed * delta * cam.front().cross(up).normalize(),

                        &Key::Escape => {
                            context.window().set_cursor_grab(false).unwrap();
                            context.window().set_cursor_visible(true);
                            mouse_locked = false;
                        },
                        _ => {}
                    }
                }

                if input_manager.is_key_pressed(Key::Equals) {
                    cam.set_sensitivity(sensitivity + 0.05)
                }

                if input_manager.is_key_pressed(Key::Subtract) {
                    cam.set_sensitivity(sensitivity - 0.05)
                }

                if input_manager.is_key_pressed(Key::E) {
                    println!("{:?}", pos * 4.0);
                }

                if !mouse_locked {
                    input_manager.suspend_input();
                }

                let (delta_x, delta_y) = input_manager.get_mouse_delta(); {
                    cam.rotate_by_mouse(delta_x as f32, delta_y as f32, delta);
                };

                context.window().request_redraw();
            },

            Event::DeviceEvent { device_id, event, .. } => {
                input_manager.update_inputs(device_id, event);
            }

            Event::RedrawRequested(_id) => {
                r.render(time, cam.generate_view());

                time += 1.0;
                std::thread::sleep(std::time::Duration::from_micros(4167/*16667*/));
                context.swap_buffers().unwrap();
                let now = Instant::now();
                delta = (now - last_time).as_secs_f32();
                last_time = now;
            },

            _ => {
                // do nothing
            }
        };
    });
}
