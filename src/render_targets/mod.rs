use crate::appdata::scene::*;
use crate::backends::my_gl;
use image::*;
use std::path::Path;

trait RenderTarget {
    fn render(&mut self, scene: Scene);
}

#[allow(dead_code)]
struct ImageRenderTarget {
    image: RgbaImage,
}

impl ImageRenderTarget {
    #[allow(dead_code)]
    fn save_image(&self, path: &Path) -> std::io::Result<()> {
        self.image.save(path)
    }

    #[allow(dead_code)]
    fn new(width: u32, height: u32) -> ImageRenderTarget {
        ImageRenderTarget {
            image: RgbaImage::new(width, height),
        }
    }
}

impl RenderTarget for ImageRenderTarget {
    fn render(&mut self, scene: Scene) {
        for model in scene.models {
            let color = image::Rgba([0, 0, 0, 0]);
            let result = my_gl::draw_mesh(model.mesh, &mut self.image, color);
            assert!(result.is_ok());
        }
    }
}
