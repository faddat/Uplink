use dioxus::prelude::*;
use ui_kit::{
    components::{
        context_menu::ContextItem,
        context_menu::ContextMenu,
        indicator::{Platform, Status},
        user_image::UserImage,
    },
    elements::{
        button::Button,
        label::Label,
        tooltip::{ArrowPosition, Tooltip},
        Appearance,
    },
    icons::Icon,
};

use crate::state::{Action, State};

#[derive(Props)]
pub struct Props<'a> {
    // The username of the friend request sender
    username: String,
    // A suffix to the username, typically a unique identifier
    suffix: String,
    // The user image element to display
    user_image: Element<'a>,
    // An optional event handler for the "onchat" event
    #[props(optional)]
    onchat: Option<EventHandler<'a>>,
    // An optional event handler for the "onremove" event
    #[props(optional)]
    onremove: Option<EventHandler<'a>>,
    // An optional event handler for the "onblock" event
    #[props(optional)]
    _onblock: Option<EventHandler<'a>>,
}

#[allow(non_snake_case)]
pub fn Friend<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    cx.render(rsx!(
        div {
            class: "friend",
            &cx.props.user_image,
            div {
                class: "request-info",
                p {
                    "{cx.props.username}",
                    span {
                        "#{cx.props.suffix}"
                    }
                },
                Label {
                    text: "Requested 4 days ago.".into()
                }
            },
            div {
                class: "request-controls",
                Button {
                    icon: Icon::ChatBubbleBottomCenterText,
                    text: "Chat".into(),
                    onpress: move |_| match &cx.props.onchat {
                        Some(f) => f.call(()),
                        None    => {},
                    }
                },
                Button {
                    icon: Icon::XMark,
                    appearance: Appearance::Secondary,
                    onpress: move |_| match &cx.props.onremove {
                        Some(f) => f.call(()),
                        None    => {},
                    }
                    tooltip: cx.render(rsx!(
                        Tooltip {
                            arrow_position: ArrowPosition::Right,
                            text: String::from("Remove Friend")
                        }
                    )),
                },
                Button {
                    icon: Icon::EllipsisVertical,
                    appearance: Appearance::Secondary,
                    onpress: move |_| {},
                    tooltip: cx.render(rsx!(
                        Tooltip {
                            arrow_position: ArrowPosition::Right,
                            text: String::from("Right click for more")
                        }
                    )),
                }
            }
        }
    ))
}

#[allow(non_snake_case)]
pub fn Friends(cx: Scope) -> Element {
    let state: UseSharedState<State> = use_context::<State>(&cx).unwrap();
    let friends = state.read().get_friends_by_first_letter();

    cx.render(rsx! (
        div {
            class: "friends-list",
            Label {
                text: "Friends".into(),
            },
            friends.into_iter().map(|(letter, sorted_friends)| {
                let group_letter = letter.to_string();
                rsx!(
                    div {
                        key: "friend-group-{group_letter}",
                        Label {
                            text: letter.into(),
                        },
                        sorted_friends.into_iter().map(|friend| {
                            let did = friend.did_key().clone();
                            let did_suffix: String = did.to_string().chars().rev().take(6).collect();
                            let chat_with_friend = state.read().get_chat_with_friend(&friend.clone());
                            let chat_with_friend_context = state.read().get_chat_with_friend(&friend.clone());
                            rsx!(
                                ContextMenu {
                                    id: format!("{}-friend-listing", did),
                                    key: "{did}-friend-listing",
                                    items: cx.render(rsx!(
                                        ContextItem {
                                            icon: Icon::ChatBubbleBottomCenterText,
                                            text: String::from("Chat"),
                                            onpress: move |_| {
                                                let _ = &state.write().mutate(Action::ChatWith(chat_with_friend_context.clone()));
                                                use_router(&cx).replace_route("/", None, None);
                                            }
                                        },
                                        ContextItem {
                                            icon: Icon::PhoneArrowUpRight,
                                            text: String::from("Call"),
                                            // TODO: Wire this up to state
                                        },
                                        ContextItem {
                                            icon: Icon::Heart,
                                            text: String::from("Favorite"),
                                            // TODO: Wire this up to state
                                        },
                                        hr{}
                                        ContextItem {
                                            danger: true,
                                            icon: Icon::XMark,
                                            text: String::from("Remove User"),
                                            // TODO: Wire this up to state\
                                        },
                                        ContextItem {
                                            danger: true,
                                            icon: Icon::NoSymbol,
                                            text: String::from("Block User"),
                                            // TODO: Wire this up to state
                                        },
                                    )),
                                    Friend {
                                        username: friend.username(),
                                        suffix: did_suffix,
                                        user_image: cx.render(rsx! (
                                            UserImage {
                                                platform: Platform::Desktop,
                                                status: Status::Online,
                                                image: friend.graphics().profile_picture()
                                            }
                                        )),
                                        onchat: move |_| {
                                            let _ = &state.write().mutate(Action::ChatWith(chat_with_friend.clone()));
                                            use_router(&cx).replace_route("/", None, None);
                                        }
                                    }
                                }
                            )
                        })
                    }
                )
            })
        }
    ))
}