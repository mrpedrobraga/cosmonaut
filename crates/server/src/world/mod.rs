use crate::message::{Message, MessageEntry, MessageFragment, MessageId};
use crate::user::{Author, User, UserProfile, Username};
use std::cmp::PartialEq;
use chrono::DateTime;

/// A world contains all the persistent information that the server might need to access while running.
pub struct World {
    neighbourhoods: Vec<Neighbourhood>,
    users: Vec<User>,
}

impl World {
    pub fn new() -> World {
        World {
            neighbourhoods: vec![],
            users: vec![],
        }
    }

    /// Inserts a new neighbourhood into the world.
    /// Many actions in a world are executed through one of these.
    pub fn insert_neighbourhood(&mut self, neighbourhood: Neighbourhood) -> NeighbourhoodId {
        let id = neighbourhood.id.clone();
        self.neighbourhoods.push(neighbourhood);
        id
    }

    /// Inserts a new user into the world.
    pub fn insert_user(&mut self, user: User) {
        self.users.push(user);
    }

    /// Finds a user given its username.
    pub fn find_user(&self, user_id: &Username) -> Option<&User> {
        self.users.iter().find(|u| u.id.eq(user_id))
    }

    /// Finds a user profile given its username.
    pub fn find_user_profile(&self, author: &Author) -> Option<&UserProfile> {
        let user = self.find_user(&author.user_id)?;
        user.profiles.iter().find(|p| p.id.eq(&author.profile_id))
    }

    /// Inserts a profile in an user. These are necessary for doing actions.
    pub fn insert_user_profile(
        &mut self,
        user_id: Username,
        profile: UserProfile,
    ) -> Result<(), ()> {
        if let Some(user) = self.users.iter_mut().find(|u| u.id == user_id) {
            user.profiles.push(profile);
            Ok(())
        } else {
            Err(())
        }
    }

    /// Inserts a new message into the world.
    pub fn insert_message(&mut self, neighbourhood_id: &NeighbourhoodId, message: Message) {
        if let Some(neighbourhood) = self
            .neighbourhoods
            .iter_mut()
            .find(|neighbourhood| neighbourhood.id.eq(neighbourhood_id))
        {
            neighbourhood.messages.push(MessageEntry {
                id: MessageId::new(),
                payload: message,
                creation_time: chrono::Utc::now(),
            });
        }
    }

    pub fn debug_neighbourhood(&self, neighbourhood_id: &NeighbourhoodId) {
        if let Some(neighbourhood) = self
            .neighbourhoods
            .iter()
            .find(|neighbourhood| neighbourhood.id.eq(neighbourhood_id))
        {
            println!(
                "Debugging {} ({}).\n---",
                neighbourhood.name, neighbourhood.id.0
            );

            for message_entry in neighbourhood.messages.iter() {
                self.debug_message(message_entry)
            }
        }
    }

    pub fn debug_message(&self, message_entry: &MessageEntry) {
        let profile = self
            .find_user_profile(&message_entry.payload.sender)
            .unwrap();
        print!(
            "{} @{}/{} : ",
            profile.display_name,
            message_entry.payload.sender.user_id.0,
            message_entry.payload.sender.profile_id.0
        );
        for part in message_entry.payload.content.0.iter() {
            match part {
                MessageFragment::Text(text) => print!("{}", text),
            }
        }
        println!();
    }
}

/// Neighbourhoods are spaces in which you can talk and have fun.
pub struct Neighbourhood {
    id: NeighbourhoodId,
    name: String,
    messages: Vec<MessageEntry>,
}

impl Neighbourhood {
    pub fn new(id: &str, name: &str) -> Neighbourhood {
        Neighbourhood {
            id: NeighbourhoodId(id.to_string()),
            name: name.to_string(),
            messages: vec![],
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct NeighbourhoodId(String);
