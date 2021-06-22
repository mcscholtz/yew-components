use yew_common::traits::{ToClass, View};
use yew::{html, Html};

use crate::icons::icon::{Icon, Size};

use fa_defines::{Layout, Props};

#[derive(Clone, PartialEq, Default)]
pub struct StackedIcon {
    pub top: Icon,
    pub bottom: Icon,
    pub size: Option<Size>,
}

impl StackedIcon {
    pub fn new(top: Icon, bottom: Icon) -> Self {
        let mut t = top;
        t.props.push(Props::Layout(Layout::StackTop));
        let mut b = bottom;
        b.props.push(Props::Layout(Layout::StackBottom));

        StackedIcon {
            top: t,
            bottom: b,
            size: None,
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }
}

impl View for StackedIcon {
    fn view(&self) -> Html {
        if let Some(size) = &self.size {
            let class = format!("fa-stack {}", size.to_class());
            html! {
                <span class=class>
                 {self.bottom.view()}
                 {self.top.view()}
                </span>
            }
        } else {
            html! {
                <span class="fa-stack">
                {self.bottom.view()}
                {self.top.view()}
               </span>
            }
        }
    }
}
