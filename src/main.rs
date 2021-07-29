use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(hello_person.system())
        .run();
}

struct Person;
struct Name(String);

fn hello_world(){
    println!("hello world!");
}

fn hello_person(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

fn add_people(mut commands: Commands){
    commands.spawn().insert(Person).insert(Name("Hanabi".to_string()));
    commands.spawn().insert(Person).insert(Name("Rem".to_string()));
    commands.spawn().insert(Person).insert(Name("Rory".to_string()));
}
