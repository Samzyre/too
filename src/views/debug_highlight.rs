use std::collections::VecDeque;

use crate::math::{Rect, Size, Space};
use crate::view::{Layout, Palette, StyleKind, Ui};
use crate::{
    renderer::Rgba,
    view::{Builder, Render, View},
};

pub type DebugHighlightClass = fn(&Palette) -> DebugHighlightStyle;

#[derive(Copy, Clone)]
pub struct DebugHighlightStyle {
    pub focus: Rgba,
    pub hover: Rgba,
    pub selection: Rgba,
}

impl DebugHighlightStyle {
    pub fn default(_palette: &Palette) -> Self {
        Self {
            focus: Rgba::hex("#8088"),
            hover: Rgba::hex("#8808"),
            selection: Rgba::hex("#0888"),
        }
    }
}

#[derive(Debug)]
pub struct Marker {
    rect: Rect,
    focused: bool,
    hovered: bool,
    selected: bool,
}

#[must_use = "a view does nothing unless `show()` or `show_children()` is called"]
pub struct DebugHighlight {
    markers: Vec<Marker>,
    class: StyleKind<DebugHighlightClass, DebugHighlightStyle>,
}

impl DebugHighlight {
    pub const fn class(mut self, class: DebugHighlightClass) -> Self {
        self.class = StyleKind::Deferred(class);
        self
    }

    pub const fn style(mut self, style: DebugHighlightStyle) -> Self {
        self.class = StyleKind::Direct(style);
        self
    }
}

impl DebugHighlight {
    fn collect_markers(&mut self, layout: Layout) {
        let root = layout.nodes.current();
        let Some(offset) = layout.layout.rect(root).map(|r| r.left_top()) else {
            return;
        };
        let mut queue = VecDeque::from([(root, offset)]);
        while let Some((id, mut offset)) = queue.pop_front() {
            if id != root {
                let Some(rect) = layout.layout.rect(id) else {
                    continue;
                };
                self.markers.push(Marker {
                    rect: rect.translate(offset.to_vec2()),
                    focused: layout.input.is_focused(id),
                    hovered: layout.input.is_hovered(id),
                    selected: layout.input.selection().map_or(false, |s| s == id),
                });
                offset += rect.left_top(); // keep track of absolute position
            }
            if let Some(node) = layout.nodes.get(id) {
                queue.extend(
                    node.children
                        .iter()
                        .copied()
                        .zip(std::iter::from_fn(|| Some(offset))),
                );
            };
        }
    }
}

impl std::fmt::Debug for DebugHighlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DebugHighlight")
            .field("markers", &self.markers)
            // .field("class", &self.class)
            .finish()
    }
}

impl<'v> Builder<'v> for DebugHighlight {
    type View = Self;
}

impl View for DebugHighlight {
    type Args<'v> = Self;
    type Response = ();

    fn create(args: Self::Args<'_>) -> Self {
        args
    }

    fn update(&mut self, args: Self::Args<'_>, ui: &Ui) -> Self::Response {
        *self = args;
    }

    fn layout(&mut self, mut layout: Layout, space: Space) -> Size {
        // default layout stuff
        let current = layout.nodes.get_current();
        let mut size = Size::ZERO;
        for &child in &current.children {
            size = size.max(layout.compute(child, space))
        }
        self.collect_markers(layout);
        space.fit(size)
    }

    fn draw(&mut self, mut render: Render) {
        // default draw stuff
        let current = render.nodes.get_current();
        for &child in &current.children {
            render.draw(child)
        }

        for Marker {
            rect,
            focused,
            hovered,
            selected,
        } in self.markers.drain(..)
        {
            let style = match self.class {
                StyleKind::Deferred(style) => style(render.palette),
                StyleKind::Direct(style) => style,
            };
            // TODO these could use blend instead to show overlapping states
            if focused {
                render.patch_bg(rect, style.focus);
            }
            if hovered {
                render.patch_bg(rect, style.hover);
            }
            if selected {
                render.patch_bg(rect, style.selection);
            }
        }
    }
}

pub fn debug_highlight() -> DebugHighlight {
    DebugHighlight {
        markers: Vec::new(),
        class: StyleKind::deferred(DebugHighlightStyle::default),
    }
}
