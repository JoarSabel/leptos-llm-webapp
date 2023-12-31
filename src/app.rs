use std::cell::RefCell;
use std::rc::Rc;

use futures::stream::SplitSink;
use leptos::*;
use leptos_meta::*;

mod components;
use components::chat_area::ChatArea;
use components::type_area::TypeArea;
use components::side_bar::SideBar;

use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App(cx: Scope) -> impl IntoView {

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (conversation, set_conversation) = create_signal(cx, Conversation::new());

    use gloo_net::websocket::futures::WebSocket;
    use gloo_net::websocket::Message::Text as Txt;
    use futures::{SinkExt, StreamExt};
    let client: Rc<RefCell<Option<SplitSink<WebSocket, gloo_net::websocket::Message>>>> = Default::default();
    let client_clone = client.clone();

    // PRAISE BE, in leptos 0.5 we wont have to pass cx all the time!
    create_effect(cx, move |_| {
        let location = web_sys::window().unwrap().location();
        let hostname = location.hostname().expect("Err-oar, failed to get origin hostname");
        let websocket_url = format!("ws://{hostname}:8080/websocket");

        let connection = WebSocket::open(&format!("{websocket_url}")).expect("Failed to do websocketeering");
        let (sender, mut recv) = connection.split();
        spawn_local(async move {
            while let Some(msg) = recv.next().await {
                match msg {
                    Ok(Txt(msg)) => {
                        set_conversation.update(move |c| {
                            c.messages.last_mut().unwrap().text.push_str(&msg);
                        });
                    }
                    _ => { break; }
                }
            }
        });
        *client_clone.borrow_mut() = Some(sender);
    });
     
    let send = create_action(cx, move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);

        });
        let client2 = client.clone();
        let msg = new_message.to_string();
        async move {
            client2
                .borrow_mut()
                .as_mut()
                .unwrap()
                .send(Txt(msg.to_string()))
                .await
                .map_err(|_| ServerFnError::ServerError("WebSocket problem".to_string()))
        }
    });
    
    create_effect(cx, move |_| {
        // Fires every time the read signal changes
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::new(),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="The Sanctum of Heinz-Werner"/>
        <div class="min-h-screen bg-gray-100">
            <div class="flex flex-row w-full h-screen">
                <div class="w-1/6">
                   <SideBar/> 
                </div>
                <div class="flex flex-col w-5/6">
                    <ChatArea conversation/>
                    <div class="flex flex-row">
                        <TypeArea send/>
                    </div>
                </div>
            </div>
        </div>
    }
}

