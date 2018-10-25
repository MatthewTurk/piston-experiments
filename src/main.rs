extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, Texture, TextureSettings };
use std::path::Path;

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    rotation_speed: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs, image_texture: &opengl_graphics::Texture) {
        // I didn't know we could do use's inside functions
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        // 0, 0, 50?
        // https://docs.rs/piston2d-graphics/0.21.1/graphics/rectangle/struct.Rectangle.html
        let square = rectangle::square(0.0, 0.0, 50.0);

        let image = Image::new().rect(square);

        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        // It seems that we are actually supplying a closure to call on the draw operation, right?
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen to GREEN
            // I think "clear" comes from graphics here
            clear(BLACK, gl);

            // this transform translates to the center, rotates, then moves by -25?
            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Now draw from the rectangle object
            image.draw(image_texture, &DrawState::default(), transform, gl);
            //rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Update our rotation by 2 radians a second
        self.rotation += self.rotation_speed * args.dt;
    }

}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "spinny thingie",
        [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // now make a new app and give it a new opengl
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        rotation_speed: 2.0
    };

    let image_texture = Texture::from_path(Path::new("testing.jpg"), &TextureSettings::new()).unwrap();
    let mut events = Events::new(EventSettings::new());
    // We're embedding a "let" into the conditional of the while loop
    // https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
    while let Some(e) = events.next(&mut window) {
        // Not totally sure, but maybe this is called whenever a render needs to happen?  Or when
        // it *does* happend?
        if let Some(r) = e.render_args() {
            app.render(&r, &image_texture);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::A => {
                        app.rotation_speed += 0.1;
                        println!("Speeding up to {:?}", app.rotation_speed)
                    },
                Key::B => {
                        app.rotation_speed -= 0.1;
                        println!("Slowing down to {:?}", app.rotation_speed)
                    },
                _ => println!("You pressed {:?}\n", key)
            }
        }
    }
}
