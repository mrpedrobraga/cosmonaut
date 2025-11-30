pub struct User {
    pub id: Username,
    pub profiles: Vec<UserProfile>,
    pub current_profile: Option<Username>,
}

impl User {
    pub fn new(username: &str, display_name: &str) -> Self {
        User {
            id: Username(username.to_string()),
            profiles: vec![
                UserProfile::new("default", display_name, Vec::default(), "#ffffff")
            ],
            current_profile: None,
        }
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Username(pub String);

impl From<&str> for Username {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

pub struct UserProfile {
    pub id: Username,
    pub display_name: String,
    pub description: String,
    pub pronouns: Vec<String>,
    pub color: String,
}

impl UserProfile {
    pub fn new(username: &str, display_name: &str, pronouns: Vec<String>, color: &str) -> Self {
        UserProfile {
            id: Username(username.to_string()),
            display_name: display_name.to_string(),
            description: "".to_string(),
            pronouns,
            color: color.to_string(),
        }
    }
}

pub struct Author {
    pub user_id: Username,
    pub profile_id: Username,
}

impl Author {
    pub fn representing(user_id: Username, profile_id: Username) -> Self {
        Self { user_id, profile_id }
    }
}