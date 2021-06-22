use yew_common::traits::{ToClass, ToDynClass, ToStyle, View};
use yew::{
    html,
    virtual_dom::VNode,
    Html,
};

pub use fa_defines::{
    Animation, Icons, Layout, Props as IconProps, Rotation, Size, Style, Transform,
};

pub use bulma_defines::{Color, Style as Css};

//TODO: span item text-icon, icon, stacked icon, masked icon, etc etc

#[derive(Clone, PartialEq, Default)]
pub struct Icon {
    pub props: Vec<IconProps>,
    pub style: Option<Vec<Css>>,
    pub transform: Option<Vec<Transform>>,
}

impl Icon {
    pub fn add_prop(self, prop: IconProps) -> Self {
        let mut new = self;
        new.props.push(prop);
        new
    }

    pub fn add_css(self, style: Css) -> Self {
        let mut new = self;
        let res = 
            if let Some(mut css) = new.style {
                css.push(style);
                css
            } else {
                vec![style]
            };
        
        new.style = Some(res);
        new
    }

    pub fn add_transform(self, transform: Transform) -> Self {
        let mut new = self;
        let res = 
            if let Some(mut transforms) = new.transform {
                transforms.push(transform);
                transforms
            } else {
                vec![transform]
            };
        
        new.transform = Some(res);
        new
    }

    fn fmt_classes(&self) -> String {
        self.props
            .iter()
            .map(|prop: &IconProps| prop.to_class())
            .collect::<Vec<&'static str>>()
            .join(" ")
    }
}

impl View for Icon {
    fn view(&self) -> Html {
        let mut h = html! {
            <i class=self.fmt_classes()></i>
        };

        //add transforms, if there are any
        if let Some(xforms) = &self.transform {
            let transform = xforms
                .iter()
                .map(|xform: &Transform| xform.to_class())
                .collect::<Vec<String>>()
                .join(" ");

            if let VNode::VTag(mut vtag) = h {
                vtag.add_attribute("data-fa-transform", transform);
                h = VNode::VTag(vtag);
            } else {
                panic!("vtag expected")
            }
        }

        //add style if there is any
        if let Some(css) = &self.style {
            let inline_css = css
                .iter()
                .map(|s: &Css| s.to_style())
                .collect::<Vec<&'static str>>()
                .join(";");

            if let VNode::VTag(mut vtag) = h {
                vtag.add_attribute("style", inline_css);
                h = VNode::VTag(vtag);
            } else {
                panic!("vtag expected")
            }
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::virtual_dom::{VNode, VTag};

    // helper function extracting class info
    fn get_class_str(vtag: &VTag) -> &str {
        vtag.attributes
            .iter()
            .find(|(k, _)| k == &"class")
            .map(|(_, v)| AsRef::as_ref(v))
            .unwrap_or("")
    }

    // make sure the classes are correct
    fn assert_classes(html: &Html, classes: Vec<&'static str>) {
        if let VNode::VTag(vtag) = html {
            classes
                .iter()
                .for_each(|class| assert!(get_class_str(&vtag).contains(class)));
        } else {
            panic!("vtag expected");
        }
    }

    // helper function extracting class info
    fn get_transform_str(vtag: &VTag) -> &str {
        vtag.attributes
            .iter()
            .find(|(k, _)| k == &"data-fa-transform")
            .map(|(_, v)| AsRef::as_ref(v))
            .unwrap_or("")
    }

    // make sure the transform are correct
    fn assert_transform(html: &Html, transform: Vec<&'static str>) {
        if let VNode::VTag(vtag) = html {
            transform
                .iter()
                .for_each(|class| assert!(get_transform_str(&vtag).contains(class)));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn test_basic_icon() {
        let props = vec![
            IconProps::Icon(Icons::Check),
            IconProps::Style(Style::Solid),
        ];
        let icon = Icon {
            props,
            style: None,
            transform: None,
        };

        assert_classes(&icon.view(), vec!["fas", "fa-check"])
    }

    #[test]
    fn test_advanced_icon() {
        let props = vec![
            IconProps::Icon(Icons::Check),
            IconProps::Style(Style::Solid),
            IconProps::FixedWidth,
            IconProps::Size(Size::X3),
            IconProps::Rotation(Rotation::Rotate180),
            IconProps::Animation(Animation::Pulse),
            IconProps::Layout(Layout::Inverse),
        ];
        let transform = vec![
            Transform::Grow(3),
            Transform::Right(5),
            Transform::Rotate(-33),
            Transform::FlipH,
        ];
        let icon = Icon {
            props,
            style: Some(vec![Css::Color(Color::Danger)]),
            transform: Some(transform),
        };

        assert_classes(
            &icon.view(),
            vec![
                "fas",
                "fa-check",
                "fa-fw",
                "fa-3x",
                "fa-rotate-180",
                "fa-pulse",
                "fa-inverse",
            ],
        );

        assert_transform(
            &icon.view(),
            vec!["grow-3", "right-5", "rotate--33", "flip-h"],
        )
    }
}
