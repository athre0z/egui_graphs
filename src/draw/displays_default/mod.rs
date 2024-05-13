mod edge;
mod edge_shape_builder;
mod node;

pub use edge::DefaultEdgeShape;
pub use node::DefaultNodeShape;

/// Clamps font size to reasonable extrema.
/// 
/// Font rendering has a tendency to scale quadratically. egui allocates and populates 
/// corresponding textures and those end up being extremely large farily quickly. On
/// some platforms we have seen crashes from allocating those textures starting at sizes 
/// around 2000. Values >200 already take very long to generate (multiple seconds).
fn clamp_font_size(size: f32) -> f32 {
    if size.is_finite() {
        size.clamp(1.0, 128.0)
    } else {
        // Infinity and NaN values tend to crash the font renderer: default to `1.0`.
        1.0
    }
}
