use leptos::{*, html::Div};

#[component]
pub fn SideBar(cx: Scope) -> impl IntoView {
    let side_bar_ref = create_node_ref::<Div>(cx);
    view! {cx,
        <div 
            class="h-screen pb-24 w-full flex flex-col border border-gray-900 bg-gray-800 items-center justify-center" 
            node_ref=side_bar_ref
        >
            <div class="flex flex-col justify-between items-center">
                <div class="mb-16 mt-1">
                    <h1>Heinz-Werners Sanctum</h1>
                    <span> Gain access to the secret! </span>
                </div>
                <div class="flex-1 justify-center items-center mx-1">
                    <h1 class="mb-4"> Disclaimers </h1>
                    <ul>
                        <li class="mb-2">
                            <span class="">
                                This site uses a uncensored open-source model, meaning the author takes no responsibility
                                for what the model may say and or suggest. Think of it as a car, you can run it off the road
                                and drive it into a tree, but you probably should not.
                            </span>
                        </li>
                        <li class="mb-2">
                            <span>
                                There are some limitations native to the model interaction wrapper, most notably if you chat long enough
                                with the bot it will no longer be able to process all the previous chats... and just die, so you might have
                                to reload the page, sorry, this is the reality. (You cold see it as a challange)
                            </span>
                        </li>
                        <li class="mb-2">
                            <span>
                                The response time might be slow, ESPECIALLY for the first response, this is hosted on our 
                                own servers so we do not have access to any fancy GPU accelerated models etc. We apologise
                                in advance.
                            </span>
                        </li>
                        <li class="mb-2">
                            <span>
                                Models tend to be... erratic, each session might be different from the last, keep that
                                in mind.
                            </span>
                        </li>
                    </ul>
                </div>
            </div>

        </div>
    }
}
