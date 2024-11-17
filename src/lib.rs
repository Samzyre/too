//! too -- a different kind of tui library
//!
//! # Simple examples
//! ## Centering some text:
//! ```rust,no_run
//! fn main() -> std::io::Result<()> {
//!     too::run(|ui| {
//!         ui.center(|ui| ui.label("hello world"));
//!     })
//! }
//! ```
//! ## A pair of buttons to increment and decrement a counter
//! ```rust,no_run
//! fn main() -> std::io::Result<()> {
//!     let mut counter = 0;
//!     too::run(|ui| {
//!         ui.vertical(|ui|{
//!             ui.horizontal(|ui|{
//!                 if ui.button("add 1").clicked() {
//!                     counter += 1;
//!                 }
//!                 if ui.button("subtract 1").clicked() {
//!                     counter -= 1;
//!                 }
//!             });
//!             ui.label(counter)
//!         });
//!     })
//! }
//! ```
//! ## Storing state in a struct
//! ```rust,no_run
//! use too::view::Ui;
//!
//! #[derive(Default)]
//! struct App {
//!     value: f32
//! }
//!
//! impl App {
//!     fn view(&mut self, ui: &Ui) {
//!         ui.slider(&mut value);
//!     }
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     let mut app = App::default()
//!     too::run(|ui| app.view(ui))
//! }
//! ```
//! ## Storing state seperately from an application
//! ```rust,no_run
//! use too::view::Ui;
//!
//! #[derive(Default)]
//! struct State {
//!     value: f32
//! }
//!
//! struct App ;
//!
//! impl App {
//!     fn view(&self, state: &mut State, ui: &Ui) {
//!         ui.slider(&mut state.value);
//!     }
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     let app = App;
//!     let mut state = State::default();
//!     too::run(|ui| app.view(&mut state, ui))
//! }
//! ```
//!
//! Some pre-made views are provided in: [`too::views`](crate::views)
//!
pub mod animation;
pub mod backend;
pub mod layout;
pub mod math;
pub mod renderer;

pub mod view;
pub mod views;

pub mod lock;

#[cfg(feature = "terminal")]
pub mod term;

mod hasher;
pub mod helpers;

#[macro_use]
mod str;
pub use str::Str;

#[doc(hidden)]
pub use compact_str::format_compact as __dont_use_this_because_semver;

#[cfg(feature = "terminal")]
mod run;
#[cfg(feature = "terminal")]
pub use run::{application, run, RunConfig};
