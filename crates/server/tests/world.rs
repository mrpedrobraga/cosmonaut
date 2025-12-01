use cosmonaut_server::{
    message::{MessageFragment, MessagePayload},
    user::{Author, User, UserProfile, Username},
    world::{Neighbourhood, World},
};

extern crate cosmonaut_server;

macro_rules! message {
    ($world:expr, $neighbourhood_id:expr; $senderu:expr, $senderp:expr => $content:expr) => {
        $world.insert_message(
            $neighbourhood_id,
            MessagePayload::new(
                Author::representing(Username::from($senderu), Username::from($senderp)),
                vec![MessageFragment::Text($content.into())],
            ),
        );
    };
}

#[test]
fn test_world() {
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

    message!(world, &n_id; "mrpedrobraga", "default" => "hi");
    message!(world, &n_id; "savestar", "minidev" => "hi!!! actually, wait");
    message!(world, &n_id; "savestar", "daisy" => "yeah, hi there");
    message!(world, &n_id; "savestar", "daisy" => "also sorry if i don't respond i have lung cancer and my mom stabbed megit ");

    world.debug_neighbourhood(&n_id);
}
