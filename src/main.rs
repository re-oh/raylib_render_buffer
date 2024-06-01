use raylib::prelude::*;
use rayrust::framer::{Framer, GlobalData, RenderFlag, Renderable};
use rayrust::math::in_bounds_vec2;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;

mod rayrust;

impl Renderable for Player {
    fn flag(&self, _draw: &mut RaylibDrawHandle) -> RenderFlag {
        RenderFlag::Render
    }

    fn render(&self, draw: &mut RaylibDrawHandle) {
        draw.draw_circle_v(self.pos, self.radius, self.color);
    }

    fn update(&mut self, draw: &mut RaylibDrawHandle) {
        self.pos = draw.get_mouse_position();
        if in_bounds_vec2(self.pos, 50.0, 500.0, 70.0, 100.0) {
            self.color = Color::RED;
        } else {
            self.color = Color::WHITE;
        }
    }
}

struct Player {
    radius: f32,
    pos: Vector2,
    color: Color,
}

// impl Player {
//     fn is_colliding_with_mouse(&self, _draw: RaylibDrawHandle) {}
// }

impl GlobalData for Global {}
#[derive(Clone, Copy)]
struct Global {
    mouse_pos: Vector2,
    counter: i32,
}

fn main() {
    let (mut rl, rlt) = raylib::init()
        .vsync()
        .width(WIDTH)
        .height(HEIGHT)
        .title("raylib-5.0 // some shit i be doin")
        .build();

    let mut global = Global {
        mouse_pos: Vector2::new(0.0, 0.0),
        counter: 0,
    };
    let mut buffer = Framer::new::<Global>(global);

    let player = Player {
        radius: 7.5,
        pos: Vector2::new(400.0, 300.0),
        color: Color::WHITE,
    };

    buffer.add_renderable(player);

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&rlt);

        global.mouse_pos = draw.get_mouse_position();
        draw.clear_background(Color::BLACK);

        draw.draw_rectangle_rec(Rectangle::new(50.0, 70.0, 450.0, 30.0), Color::GREEN);

        draw.draw_fps(12, 12);
        draw.draw_text(
            global.counter.to_string().as_str(),
            12,
            30,
            20,
            Color::GREEN,
        );
        draw.draw_text(
            format!("X:{} Y:{}", global.mouse_pos.x, global.mouse_pos.y).as_str(),
            (global.mouse_pos.x) as i32,
            (global.mouse_pos.y + 12.0) as i32,
            20,
            Color::WHITE,
        );

        buffer.iter(&mut draw);
        global.counter += 1;
    }
}
