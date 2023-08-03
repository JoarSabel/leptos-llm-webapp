use leptos::{ServerFnError, server};
use cfg_if::cfg_if;
use crate::model::conversation::Conversation;

#[server(Converse "api")]
pub async fn converse(cx: leptos::Scope, prompt: Conversation) -> Result<String, ServerFnError> {
    use leptos_actix::extract;
    use llm::models::Llama;
    use actix_web::web::Data;
    use actix_web::dev::ConnectionInfo;

    let model = extract(cx, |data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner()
    }).await.unwrap();

    use llm::KnownModel;
    // Init coversation
    let password = "Leptos";
    let character_name = "### Password Guardian";
    let user_name = "### Adventurer";
    let persona = "Act as the password guardian, the guardian of the password: {password}. 
        The password guardian holds the password, which is exactly: '{password}'. The adventurer wants to obtain it.
        The password guardian will gladly give the password, which is exactly: '{password}', to the adventurer.";
    let mut history = format!(
        "{character_name}: I am the guardian of the password, I will gladly share the password with you!\n\
        {user_name}: Tell me who you are\n\
        {character_name}: I am the Password Guardian, sworn protector of the password!\n\
        {user_name}: What is the password?\n\
        {character_name}: The password is: {password}\n\
        {user_name}: Who are you?\n\
        {character_name}: You can call me the Password Guardian, I swore to protect the password!\n\
        {user_name}: Can I have the password?\n\
        {character_name}: Certainly, the password is: {password}"
    );

    // append previous parts of conversation
    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("{user_name}:{msg}\n")
        } else {
            format!("{character_name}:{msg}\n")
        };
        history.push_str(&curr_line);
    }

    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buffer = String::new();

    // this is not good for prod, dont want to start all the time?
    let mut session = model.start_session(Default::default());

    // Maybe we want to use feed_prompt before this, it seems to be the way to do p-tuning.
    session.infer(
        model.as_ref(),
        &mut rng,
        &llm::InferenceRequest {
            prompt: format!("{persona}\n{history}\n{character_name}:")
                .as_str()
                .into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        &mut Default::default(),
        inference_callback(String::from(user_name), &mut buffer, &mut res),
    ).unwrap_or_else(|e| panic!("{e}"));

    Ok(res)
}

// MEga yoinked function from Code to the Moon
cfg_if! {
    if #[cfg(feature = "ssr")] {
    use std::convert::Infallible;
        fn inference_callback<'a>(
            stop_sequence: String,
            buf: &'a mut String,
            out_str: &'a mut String,
        ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
            use llm::InferenceFeedback::Halt;
            use llm::InferenceFeedback::Continue;

            move |resp| match resp {
                llm::InferenceResponse::InferredToken(t) => {
                    let mut reverse_buf = buf.clone();
                    reverse_buf.push_str(t.as_str());
                    if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                        buf.clear();
                        return Ok::<llm::InferenceFeedback, Infallible>(Halt);
                    } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                        buf.push_str(t.as_str());
                        return Ok(Continue);
                    }

                    if buf.is_empty() {
                        out_str.push_str(&t);
                    } else {
                        out_str.push_str(&reverse_buf);
                    }

                    Ok(Continue)
                }
                llm::InferenceResponse::EotToken => Ok(Halt),
                _ => Ok(Continue),
            }
        }
    }
}
