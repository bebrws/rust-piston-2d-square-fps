extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use std::time::Instant;


pub struct App<'a> {
    total_frames: &'a mut usize,
    start_time: Instant,
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App<'_> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        let mut glyphes = opengl_graphics::GlyphCache::new("fonts/Roboto-Bold.ttf", (), opengl_graphics::TextureSettings::new()).unwrap();

        let fps = (*self.total_frames as f64) / (Instant::now() - self.start_time).as_secs_f64();

        // *self.total_frames += 1;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);

            // opengl_graphics::graphics::text()
            let fps_string: String = format!("FPS: {:.2}", fps);
            text::Text::new_color([0.4, 0.2, 0.3, 1.0], 12).draw(&fps_string,
                                                                 &mut glyphes,
                                                                 &c.draw_state,
                                                                 c.transform.trans(10.0, 10.0),
                                                                 gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let start_time = Instant::now();
    let mut total_frames = 0;

    
    // Create a new game and run it.
    let mut app = App {
        total_frames: &mut total_frames,
        start_time,
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };


    // let assets = find_folder::Search::ParentsThenKids(3, 3)
    // .for_folder("fonts").unwrap();
    // let ref font = assets.join("Roboto-Bold.ttf");
    // window
    // let factory = window.factory.clone();
    // let mut glyphs = Glyphs::new(font, factory).unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        *app.total_frames += 1;
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}