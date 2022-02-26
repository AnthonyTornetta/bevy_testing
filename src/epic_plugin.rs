use bevy::prelude::*;

pub struct HelloPlugin;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct GreetTimer(Timer);

fn add_people(mut commands: Commands)
{
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Joe".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Mike".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Chad".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>)
{
    if timer.0.tick(time.delta()).just_finished()
    {
        for name in query.iter()
        {
            println!("Hello, {}", name.0);
        }
    }
}

impl Plugin for HelloPlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(greet_people);
        // .add_plugin(EditorPlugin);
    }
}
