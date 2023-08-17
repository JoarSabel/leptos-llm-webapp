use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Level {
    // user name will be constant
    pub user_name: &'static str,
    // bot name will be constant
    pub bot_name: &'static str,
    pub level_count: i8,
    pub history: String,
    pub persona: String, 
    pub orders: String, 
    pub init_session: bool,
}

impl Level {
    pub fn new(
        level_count: i8, 
        history: String, 
        persona: String, 
        orders: String
    ) -> Self {
       Self {
            user_name: "### Adventurer",
            bot_name: "### Heinz-Werner Grabner",
            level_count,
            history,
            persona,
            orders,
            init_session: true,
        } 
    } 
    
}
