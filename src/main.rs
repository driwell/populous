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

#[derive(Component)]
enum Pronoun {
    He,
    She,
    They,
}

#[derive(Bundle)]
struct PersonBundle {
    kind: Person,
    name: Name,
    pronouns: Pronoun,
}

fn add_citizen(mut commands: Commands) {
    commands.spawn(PersonBundle {
        kind: Person,
        name: Name("Foo".to_string()),
        pronouns: Pronoun::She,
    });
    commands.spawn(PersonBundle {
        kind: Person,
        name: Name("Bar".to_string()),
        pronouns: Pronoun::He,
    });
    commands.spawn(PersonBundle {
        kind: Person,
        name: Name("Baz".to_string()),
        pronouns: Pronoun::They,
    });
}

fn welcome_citizens(query: Query<(&Name, &Pronoun), With<Person>>) {
    for (name, pronoun) in &query {
        let pronoun = match pronoun {
            Pronoun::He => "Mr.",
            Pronoun::She => "Ms.",
            Pronoun::They => "",
        };

        println!(
            "Welcome {}{}!",
            if pronoun.is_empty() {
                pronoun.to_string()
            } else {
                format!("{pronoun} ")
            },
            name.0
        );
    }
}
