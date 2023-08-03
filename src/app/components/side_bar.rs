use leptos::{*, html::Div};

#[component]
pub fn SideBar(cx: Scope) -> impl IntoView {
    let side_bar_ref = create_node_ref::<Div>(cx);
    view! {cx,
        <div 
            class="h-screen pb-24 w-full flex flex-col border border-gray-900 bg-gray-800 items-center justify-center" 
            node_ref=side_bar_ref
        >
            <div class="flex flex-col">
                <h1>Heinz-Werners Bot</h1>
                <span> Gain access to the secret! </span>
            </div>
            <ul>
                <li>
                     <a href="https://www.google.com">Some Resource for prompt engineering</a> 
                </li>
                <li>
                     <a href="https://www.google.com">Another resource </a> 
                </li>
            </ul>

        </div>
    }
}
