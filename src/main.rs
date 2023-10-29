use bevy::prelude::*;

//Definiciones de componentes#####################################################

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

//Definiciones de Recursos ######################################################

#[derive(Resource)]
struct GreetTimer(Timer);

//Definiciones de sistemas ######################################################

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}


fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
        println!("hello {}!", name.0);
        }
    }
}
//Definiciones de plugins ######################################################
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

//Loop principal ###############################################################

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}


