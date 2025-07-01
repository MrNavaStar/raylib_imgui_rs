use imgui::{TextureId, Ui};
use raylib::prelude::{RenderTexture2D, Texture2D};

pub trait TextureLike {
    fn id(&self) -> u32;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

impl TextureLike for Texture2D {
    fn id(&self) -> u32 {
        self.id
    }

    fn width(&self) -> u32 {
        self.width as u32
    }

    fn height(&self) -> u32 {
        self.height as u32
    }
}

impl TextureLike for RenderTexture2D {
    fn id(&self) -> u32 {
        self.texture.id
    }

    fn width(&self) -> u32 {
        self.texture.width as u32
    }

    fn height(&self) -> u32 {
        self.texture.height as u32
    }
}

pub trait ImageExt {
    fn image_scaled<T: TextureLike>(&self, texture: &T, width: u32, height: u32);
    fn image<T: TextureLike>(&self, texture: &T);
}

impl ImageExt for Ui {
    fn image_scaled<T: TextureLike>(&self, texture: &T, width: u32, height: u32) {
        imgui::Image::new(
            TextureId::new(texture.id() as _),
            [width as _, height as _],
        ).build(self);
    }

    fn image<T: TextureLike>(&self, texture: &T) {
        self.image_scaled(texture, texture.width(), texture.height());
    }
}