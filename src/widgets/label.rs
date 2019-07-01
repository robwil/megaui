use crate::{
    types::{Point2, Vector2},
    Layout, Ui,
};

use std::borrow::Cow;

pub struct Label<'a> {
    position: Option<Point2>,
    multiline: Option<f32>,
    label: Cow<'a, str>,
}

impl<'a> Label<'a> {
    pub fn new<S>(label: S) -> Label<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Label {
            position: None,
            multiline: None,
            label: label.into(),
        }
    }

    pub fn multiline(self, line_height: f32) -> Self {
        Label {
            multiline: Some(line_height),
            ..self
        }
    }

    pub fn position<P: Into<Option<Point2>>>(self, position: P) -> Self {
        let position = position.into();

        Label { position, ..self }
    }

    pub fn ui(self, ui: &mut Ui) {
        let context = ui.get_active_window_context();

        let size = Vector2::new(
            150.,
            self.multiline.map_or(14., |line_height| {
                line_height * self.label.split('\n').count() as f32
            }),
        );

        let color = context.global_style.text(context.focused);
        let pos = context
            .window
            .cursor
            .fit(size, self.position.map_or(Layout::Vertical, Layout::Free));
        if let Some(line_height) = self.multiline {
            for (n, line) in self.label.split('\n').enumerate() {
                context.window.draw_list.draw_label(
                    line,
                    pos + Vector2::new(0., n as f32 * line_height),
                    color,
                )
            }
        } else {
            context
                .window
                .draw_list
                .draw_label(&*self.label, pos, color)
        }
    }
}

impl Ui {
    pub fn label<P: Into<Option<Point2>>>(&mut self, position: P, label: &str) {
        Label::new(label).position(position).ui(self)
    }
}
