// use std::thread;
use std::sync::mpsc::{ Receiver, Sender };
// use time::{ self, Timespec };
use find_folder::{ Search };
use conrod::{ Theme, Ui, };
use piston_window::{ PistonWindow, WindowSettings, Window, Glyphs, OpenGL, Size, Key, PressEvent, Button as KeyButton };

use piston_window::{ Flip, TextureSettings, Texture, image, clear };
// use opengl_graphics::glyph_cache::GlyphCache;
// use elmesque::{ self, Form, Renderer };


use loop_cycles::{ CyCtl, CyStatus };
use super::conrod_draw;
use super::super::window_stats::{ WindowStats };


pub fn open(mut control_tx: Sender<CyCtl>, status_rx: Receiver<CyStatus>) {
    let win_size_init = Size { width: 1200, height: 800 };

    let window: PistonWindow = WindowSettings::new("Vibi", win_size_init)
        .opengl(OpenGL::V3_2)
        .exit_on_esc(true)
        .build().expect("Window build error");

    let (mut ui, rust_logo) = {
        let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone()).unwrap();

        // Rust logo image asset:
        let rust_logo = Texture::from_path(
                &mut *window.factory.borrow_mut(),
                &assets.join("rust.png"),
                Flip::None,
                &TextureSettings::new()
            ).unwrap();

        (Ui::new(glyph_cache, Theme::default()), rust_logo)
    };

    // Loop mutables:
    let mut iters = "1".to_string();
    let mut cycle_status = CyStatus::new();
    let mut close_window = false;
    let mut stats = WindowStats::new();

    for e in window {
        if let Some(KeyButton::Keyboard(key)) = e.press_args() {
            if key == Key::Q {
                println!("Pressed 'Q'.");
                close_window = true;
            }

            println!("Pressed keyboard key '{:?}'.", key);
        };

        ui.handle_event(&e.event.clone().unwrap());

        loop {
            match status_rx.try_recv() {
                Ok(cs) => {
                    cycle_status = cs;
                },
                Err(_) => break,
            };
        }

        e.draw_2d(|c, g| {
            let win_size = e.size();

            clear([0.2, 0.25, 0.4, 1.0], g);

            image(&rust_logo, c.transform, g);
            

            ///////////// NEW DRAW //////////////
            conrod_draw::draw_buttons(&mut ui, &mut cycle_status, &win_size, 
                &mut control_tx, &stats, &mut close_window, &mut iters);


            ui.draw_if_changed(c, g);
        });

        if close_window { break; }
        stats.incr();
    }
}
