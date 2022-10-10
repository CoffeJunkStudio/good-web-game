//! https://github.com/ggez/ggez/blob/master/examples/canvas_subframe.rs
//! Sprite batch is not really implemented yet, so its extremely slow.
//! But the framebuffer works fine, nice!
//!
//! An example of how to use a `SpriteBatch`.
//!

extern crate cgmath;
extern crate good_web_game as ggez;

use ggez::event;
use ggez::graphics;
use ggez::miniquad;
use ggez::timer;
use ggez::{Context, GameResult};

type Point2 = cgmath::Point2<f32>;
type Vector2 = cgmath::Vector2<f32>;

struct MainState {
    spritebatch: graphics::spritebatch::SpriteBatch,
    canvas: graphics::Canvas,
    draw_pt: Point2,
    draw_vec: Vector2,
}

impl MainState {
    fn new(ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, quad_ctx, "/tile.png").unwrap();
        let spritebatch = graphics::spritebatch::SpriteBatch::new(image);
        let canvas = graphics::Canvas::with_window_size(ctx, quad_ctx)?;
        let draw_pt = Point2::new(0.0, 0.0);
        let draw_vec = Vector2::new(1.0, 1.0);
        let s = MainState {
            spritebatch,
            canvas,
            draw_pt,
            draw_vec,
        };
        Ok(s)
    }
}

impl MainState {
    fn draw_spritebatch(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
    ) -> GameResult {
        graphics::set_canvas(ctx, Some(&self.canvas));
        graphics::clear(ctx, quad_ctx, graphics::Color::WHITE);

        // Freeze the animation so things are easier to see.
        let time = 2000;
        //let time = (timer::duration_to_f64(timer::time_since_start(ctx)) * 1000.0) as u32;
        let cycle = 10_000;
        for x in 0..150 {
            for y in 0..150 {
                let x = x as f32;
                let y = y as f32;
                let p = graphics::DrawParam::new()
                    .dest(Point2::new(x * 10.0, y * 10.0))
                    // scale: graphics::Point::new(0.0625, 0.0625),
                    .scale(Vector2::new(
                        ((time % cycle * 2) as f32 / cycle as f32 * 6.28)
                            .cos()
                            .abs()
                            * 0.0625,
                        ((time % cycle * 2) as f32 / cycle as f32 * 6.28)
                            .cos()
                            .abs()
                            * 0.0625,
                    ))
                    .rotation(-2.0 * ((time % cycle) as f32 / cycle as f32 * 6.28));
                self.spritebatch.add(p);
            }
        }

        let param = graphics::DrawParam::new()
            .dest(Point2::new(
                ((time % cycle) as f32 / cycle as f32 * 6.28).cos() * 50.0 + 150.0,
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin() * 50.0 + 250.0,
            ))
            .scale(Vector2::new(
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin().abs() * 2.0 + 1.0,
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin().abs() * 2.0 + 1.0,
            ))
            .rotation((time % cycle) as f32 / cycle as f32 * 6.28)
            // WARNING: Using an offset != (0.,0.) on a spritebatch may come with a significant performance cost.
            // This is due to the fact that the total dimensions of everything drawn by it have to be calculated.
            // See SpriteBatch::draw and SpriteBatch::dimensions for more information.
            .offset(Point2::new(0.5, 0.5));

        graphics::draw(ctx, quad_ctx, &self.spritebatch, param)?;
        self.spritebatch.clear();
        graphics::set_canvas(ctx, None);
        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(
        &mut self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::GraphicsContext,
    ) -> GameResult {
        if timer::ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?} ", timer::delta(ctx));
            println!("Average FPS: {}", timer::fps(ctx));
        }

        // Bounce the rect if necessary
        let (w, h) = graphics::drawable_size(quad_ctx);
        if self.draw_pt.x + (w as f32 / 2.0) > (w as f32) || self.draw_pt.x < 0.0 {
            self.draw_vec.x *= -1.0;
        }
        if self.draw_pt.y + (h as f32 / 2.0) > (h as f32) || self.draw_pt.y < 0.0 {
            self.draw_vec.y *= -1.0;
        }
        self.draw_pt += self.draw_vec;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.draw_spritebatch(ctx, quad_ctx)?;
        let dims = self.canvas.image().dimensions();
        let src_x = self.draw_pt.x / dims.w;
        let src_y = self.draw_pt.y / dims.h;
        graphics::draw(
            ctx,
            quad_ctx,
            &self.canvas,
            graphics::DrawParam::new()
                .dest(self.draw_pt)
                .src(graphics::Rect::new(src_x, src_y, 0.5, 0.5)),
        )?;

        graphics::present(ctx, quad_ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    ggez::start(
        ggez::conf::Conf::default().cache(Some(include_bytes!("resources.tar"))),
        |mut context, quad_ctx| Box::new(MainState::new(&mut context, quad_ctx).unwrap()),
    )
}
