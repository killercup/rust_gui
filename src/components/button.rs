use super::super::prelude::*;
use std::default::Default;

#[derive(Debug, Clone, Copy)]
pub enum ButtonEvent{
	/// click event on mouse down
    Click,
	/// double click event on mouse down
    DoubleClick,
	/// if mose move and mouse over button
    Hover,
	/// mouse goes away from button
    Leave
}

/// Usage:
///
/// ```rust
/// use rui::components::Button;
/// use std::default::Default;
///
/// ctx.add(Button{
/// 	text: "Button",
/// 	..Default::default()
/// });
/// ```
#[derive(Debug, Clone)]
pub struct Button<'a>{
    pub text: &'a str,
    pub width: f32,
    pub height: f32,
    pub background_color: (f32, f32, f32),
}

impl<'a> Default for Button<'a>{
    fn default() -> Button<'a>{
        Button{
            text: "",
            width: 100.,
            height: 100.,
            background_color: (0.5,0.5,0.5),//#4285f4
        }
    }
}

setter!(Button<'a>,
    width: f32,
    height: f32,
    text: &'a str,
    background_color: (f32, f32, f32)
);

impl<'a> Widget for Button<'a>{
    type Event = ButtonEvent;
    type State = ();

    fn render<C: Context<TWidget=Button<'a>>>(&self, c: &mut C) {
        //let hovered = c.hovered();
        println!("draw_button  {:?}", self);

        c.draw_path(Path::rect(0.,0.,self.width,self.height)
            .stroke(1., Color::rgb(180,180,180)));
        c.text(10., 10. ,self.text);
        c.on_click(|pos, h| {
            println!("Button !clicked at {:?}", pos);
            h.emit(ButtonEvent::Click);
        });

        //TODO: if focused make click on enter
        //TODO: draw label
        //TODO: optional icon
        //TODO: tooltip
    }

    fn name() -> &'static str{"Button"}
}
