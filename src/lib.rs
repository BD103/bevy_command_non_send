use bevy_ecs::{
    system::{Command, Commands},
    world::{FromWorld, World},
};

pub fn init_non_send_resource<R: FromWorld + 'static>() -> impl Command {
    |world: &mut World| {
        world.init_non_send_resource::<R>();
    }
}

pub fn insert_non_send_resource<F, R>(func: F) -> impl Command
where
    // TODO: Consider FnOnce(&mut World)?
    F: FnOnce() -> R + Send + 'static,
    R: 'static,
{
    move |world: &mut World| {
        world.insert_non_send_resource((func)());
    }
}

pub fn remove_non_send_resource<R: 'static>() -> impl Command {
    |world: &mut World| {
        world.remove_non_send_resource::<R>();
    }
}

trait CommandsExt {
    fn init_non_send_resource<R: FromWorld + 'static>(&mut self);

    fn insert_non_send_resource<F, R>(&mut self, func: F)
    where
        F: FnOnce() -> R + Send + 'static,
        R: 'static;

    fn remove_non_send_resource<R: 'static>(&mut self);
}

impl CommandsExt for Commands<'_, '_> {
    fn init_non_send_resource<R: FromWorld + 'static>(&mut self) {
        self.add(init_non_send_resource::<R>());
    }

    fn insert_non_send_resource<F, R>(&mut self, func: F)
    where
        F: FnOnce() -> R + Send + 'static,
        R: 'static,
    {
        self.add(insert_non_send_resource(func));
    }

    fn remove_non_send_resource<R: 'static>(&mut self) {
        self.add(remove_non_send_resource::<R>());
    }
}
