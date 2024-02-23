use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
    tooltip: Element,
    children: Element,
}

// TODO: Right now this component only displays tooltips below the wrapped component, in the future we should expand this component to support tooltip positions.

#[allow(non_snake_case)]
pub fn TooltipWrap<'a>(props: Props<'a>) -> Element {
    cx.render(rsx!(div {
        class: "tooltip-parent",
        div {
            class: "tooltip-wrapper",
            props.tooltip.as_ref()
        }
        props.children.as_ref()
    }))
}
