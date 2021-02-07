pub mod multicolor;

use std::rc::Rc;

use lru::LruCache;
use lyon::tessellation::VertexBuffers;

use super::super::Application;
use super::math::*;
use super::Ctx;

/// Something that can be drawn to the screen.
pub trait Geometry: Sized {
    type Vertex: Vertex;

    fn cache<A: Application>(ctx: &mut Ctx<A>) -> &mut LruCache<u64, Rc<Self>>;

    fn from_lyon<A: Application>(ctx: &Ctx<A>, bufs: &VertexBuffers<Self::Vertex, u16>, aabb: Box2D<GeomSpace>)
        -> Self;
    fn draw<A: Application>(&self, ctx: &mut Ctx<A>, transform: [[f32; 4]; 4], params: &glium::DrawParameters); // XXX: should be Transform3D
    fn bounding_box(&self) -> &Box2D<GeomSpace>;
}

pub trait Vertex {
    fn from_fill(vertex: lyon::tessellation::FillVertex) -> Self;
}
