use crate::objects::*;

trait RenderTarget {
    #[allow(unused)]
    fn render(scene: Scene);
}

#[allow(dead_code)]
struct ImageRenderTarget {
    //TODO
}

impl RenderTarget for ImageRenderTarget {
    #[allow(unused)]
    fn render(scene: Scene) {
        //TODO
    }
}
