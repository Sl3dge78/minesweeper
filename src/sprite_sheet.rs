use crate::{math::Vec2, renderer::Texture};

pub trait SpriteSheet {
    fn get_uv(&self, x: i32, y: i32) -> Vec2;
    fn get_sprite_size(&self) -> Vec2;
}

impl SpriteSheet for Texture {
    fn get_uv(&self, x: i32, y: i32) -> Vec2 {
        let nb_x = self.width / 16;
        let nb_y = self.height / 16;
        Vec2::new(x as f32 / nb_x as f32, y as f32 / nb_y as f32)
    }

    fn get_sprite_size(&self) -> Vec2 {
        let nb_x = self.width / 16;
        let nb_y = self.height / 16;
        Vec2::new(1.0 / nb_x as f32, 1.0 / nb_y as f32)
    }
}