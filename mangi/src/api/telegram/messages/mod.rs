pub struct Messages {
    pub welcome: &'static str,
    pub diet: &'static str,
    pub user_type: &'static str,
}

impl Messages {
    const fn new() -> Self {
        Self {
            welcome: include_str!("./Welcome.md"),
            diet: include_str!("./Diet.md"),
            user_type: include_str!("./UserType.md"),
        }
    }
}

#[allow(non_upper_case_globals)]
pub const mangi_messages: Messages = Messages::new();
