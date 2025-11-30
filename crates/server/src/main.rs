#[macro_use]
extern crate rocket;

use crate::message::{Message, MessageFragment};
use crate::user::{Author, User, UserProfile, Username};
use crate::world::{Neighbourhood, World};

mod message;
mod user;
mod world;

fn main() {
    // In the final app, we should restore a world from a persistent location.
    let mut world = World::new();

    let neighbourhood = Neighbourhood::new("test", "Test Neighbourhood");
    let n_id = world.insert_neighbourhood(neighbourhood);

    let user1 = User::new("mrpedrobraga", "Pedro Braga");
    world.insert_user(user1);

    let user2 = User::new("savestar", "Default");
    world.insert_user(user2);

    world
        .insert_user_profile(
            Username::from("savestar"),
            UserProfile::new("minidev", "3.14", vec!["they_them".into()], "#696969"),
        )
        .unwrap();

    world
        .insert_user_profile(
            Username::from("savestar"),
            UserProfile::new("daisy", "Daisy", vec!["he_him".into()], "#704cab"),
        )
        .unwrap();

    world.insert_message(
        &n_id,
        Message::new(
            Author::representing(Username::from("mrpedrobraga"), Username::from("default")),
            vec![MessageFragment::Text("hi bro".into())],
        ),
    );

    world.insert_message(
        &n_id,
        Message::new(
            Author::representing(Username::from("savestar"), Username::from("minidev")),
            vec![MessageFragment::Text("hi. actually wait".into())],
        ),
    );

    world.insert_message(
        &n_id,
        Message::new(
            Author::representing(Username::from("savestar"), Username::from("daisy")),
            vec![MessageFragment::Text("yeah, hi pedro".into())],
        ),
    );

    world.insert_message(
        &n_id,
        Message::new(
            Author::representing(Username::from("savestar"), Username::from("daisy")),
            vec![MessageFragment::Text(
                "also sorry if i don't respond i have lung cancer and also my mom stabbed me".into(),
            )],
        ),
    );

    world.debug_neighbourhood(&n_id);
}
