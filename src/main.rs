use ggez::event::*;
use ggez::graphics;
use ggez::*;
use std::io::Read;

struct MyGame {
    image: graphics::Image,
}
impl MyGame {
    pub fn new(ctx: &mut Context) -> Self {
        let mut text = filesystem::open(ctx, "/dir/test.txt").expect("aieeeeeeeeeeee!");
        let mut buf = String::new();
        text.read_to_string(&mut buf).unwrap();
        println!("{}", buf);

        let image = graphics::ImageGeneric::new(ctx, "/dir/Mandrill.bmp").expect("aieeeeeee!");
        Self { image }
    }
}
impl EventHandler<GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::WHITE);
        graphics::draw(ctx, &self.image, graphics::DrawParam::new())?;
        graphics::present(ctx)
    }
}

fn main() {
    let mut cb = ContextBuilder::new("my_game", "Orito");
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }
    let (mut ctx, event_loop) = cb.build().expect("aieee");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game)
}
