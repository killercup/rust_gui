use Widget;
use Context;
use Color;
use std::default::Default;
use nanovg::{LEFT,RIGHT,TOP,BOTTOM,MIDDLE, Font, Image, Ctx};
use Backend;

pub struct Label<'a>{
    pub text: &'a str,
    pub font_size: f32,
    pub font_face: &'a str,
    pub color: Color,
	pub font_blur: f32,
}

impl<'a> Label<'a>{
    pub fn new(text: &'a str) -> Label<'a>{
        Label{
            text: text,
            ..Default::default()
        }
    }
}

impl<'a> Default for Label<'a>{
    fn default() -> Label<'a>{
        Label{
            text: "Label",
            font_size: 14.0,
			font_blur: 0.,
            font_face: "sans-bold",
            color: Color::rgb(255,255,255),
        }
    }
}

///implement setter for the propertys
setter!(Label<'a>,
    text: &'a str,
    font_size: f32,
    font_face: &'a str,
	font_blur: f32,
    color: Color
);

impl<'a> Widget for Label<'a>{
    type Event = ();
    type State = ();

    fn render<C:Context<TWidget=Label<'a>>>(&self, c: &mut C, _: &()) {
        c.font_face(self.font_face);
        c.draw(|be|{
            be.font_size(self.font_size);
            be.font_color(self.color.clone());
        });
        c.text(self.font_size*0.3,self.font_size*0.5,self.text);
    }
}
