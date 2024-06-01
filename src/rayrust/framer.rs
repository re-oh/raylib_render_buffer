use raylib::prelude::*;

pub trait Renderable {
    fn flag(&self, draw: &mut RaylibDrawHandle) -> RenderFlag;
    fn render(&self, draw: &mut RaylibDrawHandle);
    fn update(&mut self, draw: &mut RaylibDrawHandle);
}

pub trait GlobalData {}

#[allow(dead_code)]
pub enum RenderFlag {
    Render,
    Jump,
    Last,
    Cull,
}

pub struct Framer {
    buffer: Vec<Box<dyn Renderable>>,
    global: Box<dyn GlobalData>,
}

impl Framer {
    pub fn new<Global>(global: impl GlobalData + 'static) -> Self {
        Self {
            buffer: Vec::new(),
            global: Box::new(global),
        }
    }

    pub fn add_renderable<T: Renderable + 'static>(&mut self, renderable: T) {
        self.buffer.push(Box::new(renderable));
    }

    pub fn iter(&mut self, draw: &mut RaylibDrawHandle) {
        let mut new_buffer: Vec<Box<dyn Renderable>> = Vec::new();

        for mut renderable in self.buffer.drain(..) {
            renderable.update(draw);

            match renderable.flag(draw) {
                RenderFlag::Render => {
                    renderable.render(draw);
                    new_buffer.push(renderable);
                }
                RenderFlag::Jump => {
                    new_buffer.push(renderable);
                }
                RenderFlag::Last => {
                    renderable.render(draw);
                }
                RenderFlag::Cull => {
                    // Do nothing
                }
            }
        }

        self.buffer = new_buffer;
    }
}
