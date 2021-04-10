use std::sync::{Arc, Mutex};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::vec3::Vec3;
use std::{thread, time};
use crate::color::scale_color;

pub fn draw_window(rx: Arc<Mutex<Vec<Vec<Vec3>>>>, width: i32, height: i32, samples_per_pixel: usize) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        let mut x = 0;
        let mut y = 0;
        let guard = rx.lock().unwrap();
        let vec = guard.clone();
        drop(guard);
        for i in vec.iter() {
            for j in i.iter() {
                let col = scale_color(j, samples_per_pixel);
                canvas.set_draw_color(Color::RGB((col.x() * 255.0) as u8, (col.y() * 255.0) as u8, (col.z() * 255.0) as u8));
                canvas.draw_point(Point::new(x, y));

                x += 1;
                if x == width {
                    y += 1;
                    x = 0;
                }
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {

                }
                _ => {}
            }
        }
        canvas.present();

        thread::sleep(time::Duration::from_secs_f32(2.0));
    }
}
