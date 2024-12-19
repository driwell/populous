use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, (add_citizen, welcome_citizens).chain())
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Bundle)]
struct PersonBundle {
    kind: Person,
    name: Name,
}

fn add_citizen(mut commands: Commands) {
    commands.spawn(PersonBundle {
        kind: Person,
        name: Name("Foo".to_string()),
    });
}

fn welcome_citizens(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Welcome {}!", name.0);
    }
}
