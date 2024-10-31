use embedded_graphics::{
    mono_font::{
        ascii::FONT_6X13, MonoFont, MonoTextStyle
    },
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Point},
    primitives::{
        Primitive, PrimitiveStyle, Rectangle
    },
    Drawable
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::widget::view::{DrawView, View};
use crate::widget::view::Widget;

pub const LABEL_FONT: MonoFont = FONT_6X13;

pub struct Label {
    widget: &'static Widget,
    text: &'static str,
}

impl Label {
    pub fn new(widget: &'static Widget, text: &'static str) -> Self {
        Label {
            widget,
            text,
        }
    }
}

impl View for Label {
    type DrawInput<'a> = ();
    type DrawOutput = ();
    type TapInput<'a> = ();
    type TapOutput = ();

    fn bounding_box(&self) -> Rectangle {
        self.widget.bounding_box()
    }

    fn bounding_box_absolut(&self) -> Rectangle {
        self.widget.bounding_box_absolute()
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, _: Self::DrawInput<'a>) -> Result<Self::DrawOutput,D::Error>
        where 
            D: DrawTarget<Color = BinaryColor>,
            Self: 'a,
        {
        let filled = PrimitiveStyle::with_fill(BinaryColor::Off);
        self.bounding_box_view()
            .into_styled(filled)
            .draw(target)?;

        let character_style = MonoTextStyle::new(&LABEL_FONT, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        TextBox::with_textbox_style(
            &self.text,
            self.bounding_box_view(),
            character_style,
            textbox_style,
        )
        .draw(target)?;

        Ok(())
    }

    fn handle_tap_view<'a>(&mut self, _: Point, _: ()) -> Self::TapOutput
    where Self: 'a {
    }
}