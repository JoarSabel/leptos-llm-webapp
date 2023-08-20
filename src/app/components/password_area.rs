use leptos::{*, html::Input};

#[component]
pub fn PasswordArea(cx: Scope, password_send: Action<String, Result<(), ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    view!{cx,
        <div class="h-24 w-full bottom-0 flex justify-center items-center p-5 bg-gradient-to-t from-gray-700 bg-opacity-40">
            <form 
                class="w-full flex justify-center items-center gap-4" 
                on:submit=move |ev| {
                    ev.prevent_default();
                    let input = input_ref.get().expect("input to exist");
                    password_send.dispatch(input.value());
                    // input.set_value("")
                }
            >
                <input 
                    class="w-2/3 p-4 border border-gray-300 rounded-full" 
                    type="text" 
                    placeholder="Guess the password" 
                    node_ref=input_ref
                />
                <input 
                    class="h-full w-8 p-4 bg-red-500 text-white rounded-full cursor-pointer" 
                    type="submit"
                />
            </form>
        </div>
    }
}   
