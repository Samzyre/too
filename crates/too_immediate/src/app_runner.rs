use crate::App;
use too_backend::{Backend, EventReader};
use too_overlay::Overlay;
use too_runner::Runner;

/// A trait to run your application
///
/// It is implemented for all types that implement [`App`].
///
/// # Example:
/// ```rust
/// use too_immediate::{AppRunner as _};
/// use too_runner::Context;
/// use too_renderer::SurfaceMut;
///
/// struct Demo {
///     state: i32
/// }
///
/// impl Demo {
///     fn new(state: i32) -> Self {
///         Self { state }
///     }
/// }
///
/// impl too_immediate::App for Demo {
///     fn render(&mut self, surface: SurfaceMut, ctx: Context) {}
/// }
///
/// # fn get_backend() -> std::io::Result<too_backend::DummyBackend> { Ok(too_backend::DummyBackend) }
/// fn main() -> std::io::Result<()> {
///     let backend = get_backend()?;
///     Demo::new(1234).run(backend)
/// }
/// ```
pub trait AppRunner: App + Sealed + Sized {
    /// Run the [`App`] with the provided [`Backend`] and [`EventReader`]
    fn run(self, backend: impl Backend + EventReader) -> std::io::Result<()> {
        Runner::new()
            .min_ups(Self::min_ups)
            .max_ups(Self::max_ups)
            .init(Self::initial_size)
            .event(Self::event)
            .update(Self::update)
            .render(Self::render)
            .post_render(|ctx, overlay, surface| Overlay::default_draw(ctx, overlay, surface))
            .run(self, backend)
    }
}

#[doc(hidden)]
pub trait Sealed {}

impl<T> Sealed for T {}
impl<T: App + Sealed> AppRunner for T {}
