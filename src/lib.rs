//! This is a small utility library that enables manipulating non-[`Send`] resources using [`Commands`] in [Bevy].
//!
//! You can use this library by importing [`CommandsExt`] and calling all of its methods on [`Commands`], or you can manually call [`Commands::add`].
//! 
//! [Bevy]: https://bevyengine.org

use bevy_ecs::{
    system::{Command, Commands},
    world::{FromWorld, World},
};

/// Creates a [`Command`] for inserting a non-[`Send`] resource in the [`World`] with an inferred value.
///
/// See [`World::init_non_send_resource`] for more details.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_command_non_send::init_non_send_resource;
/// #
/// struct MyNonSend(*const u8);
///
/// impl Default for MyNonSend {
///     fn default() -> Self {
///         MyNonSend(std::ptr::null())
///     }
/// }
///
/// fn create_my_non_send(mut commands: Commands) {
///     commands.add(
///         init_non_send_resource::<MyNonSend>()
///     );
/// }
/// #
/// # App::new()
/// #     .add_systems(Startup, (create_my_non_send, check).chain())
/// #     .run();
/// #
/// # fn check(my_non_send: NonSend<MyNonSend>) {
/// #     assert!(my_non_send.0.is_null());
/// # }
/// ```
pub fn init_non_send_resource<R: FromWorld + 'static>() -> impl Command {
    |world: &mut World| {
        world.init_non_send_resource::<R>();
    }
}

/// Creates a [`Command`] for inserting a non-[`Send`] resource in the [`World`] with an specific value.
///
/// Note that this command takes a closure, not a value. This closure is executed on the main thread and should return the value of the non-[`Send`] resource. The closure itself must be [`Send`], but its returned value does not need to be.
///
/// See [`World::insert_non_send_resource`] for more details.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_command_non_send::insert_non_send_resource;
/// #
/// struct MyNonSend(*const u8);
///
/// fn create_my_non_send(mut commands: Commands) {
///     commands.add(
///         insert_non_send_resource(|| {
///             MyNonSend(std::ptr::null())
///         })
///     );
/// }
/// #
/// # App::new()
/// #     .add_systems(Startup, (create_my_non_send, check).chain())
/// #     .run();
/// #
/// # fn check(my_non_send: NonSend<MyNonSend>) {
/// #     assert!(my_non_send.0.is_null());
/// # }
/// ```
pub fn insert_non_send_resource<F, R>(func: F) -> impl Command
where
    F: FnOnce() -> R + Send + 'static,
    R: 'static,
{
    move |world: &mut World| {
        world.insert_non_send_resource((func)());
    }
}

/// Creates a [`Command`] for removing a non-[`Send`] resource from the [`World`].
///
/// See [`World::remove_non_send_resource`] for more details.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_command_non_send::remove_non_send_resource;
/// #
/// struct MyNonSend(*const u8);
///
/// fn remove_my_non_send(mut commands: Commands) {
///     commands.add(
///         remove_non_send_resource::<MyNonSend>()
///     );
/// }
/// #
/// # App::new()
/// #     .insert_non_send_resource(MyNonSend(std::ptr::null()))
/// #     .add_systems(Startup, (remove_my_non_send, check).chain())
/// #     .run();
/// #
/// # fn check(my_non_send: Option<NonSend<MyNonSend>>) {
/// #     assert!(my_non_send.is_none());
/// # }
/// ```
pub fn remove_non_send_resource<R: 'static>() -> impl Command {
    |world: &mut World| {
        world.remove_non_send_resource::<R>();
    }
}

/// Extensions to [`Commands`] that allow you to call [`init_non_send_resource`], [`insert_non_send_resource`], and [`remove_non_send_resource`].
pub trait CommandsExt: private::Sealed {
    /// See [`init_non_send_resource`].
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_command_non_send::CommandsExt;
    /// #
    /// struct MyNonSend(*const u8);
    ///
    /// impl Default for MyNonSend {
    ///     fn default() -> Self {
    ///         MyNonSend(std::ptr::null())
    ///     }
    /// }
    ///
    /// fn create_my_non_send(mut commands: Commands) {
    ///     commands.init_non_send_resource::<MyNonSend>();
    /// }
    /// #
    /// # App::new()
    /// #     .add_systems(Startup, (create_my_non_send, check).chain())
    /// #     .run();
    /// #
    /// # fn check(my_non_send: NonSend<MyNonSend>) {
    /// #     assert!(my_non_send.0.is_null());
    /// # }
    /// ```
    fn init_non_send_resource<R: FromWorld + 'static>(&mut self);

    /// See [`insert_non_send_resource`].
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_command_non_send::CommandsExt;
    /// #
    /// struct MyNonSend(*const u8);
    ///
    /// fn create_my_non_send(mut commands: Commands) {
    ///     commands.insert_non_send_resource(|| {
    ///         MyNonSend(std::ptr::null())
    ///     });
    /// }
    /// #
    /// # App::new()
    /// #     .add_systems(Startup, (create_my_non_send, check).chain())
    /// #     .run();
    /// #
    /// # fn check(my_non_send: NonSend<MyNonSend>) {
    /// #     assert!(my_non_send.0.is_null());
    /// # }
    /// ```
    fn insert_non_send_resource<F, R>(&mut self, func: F)
    where
        F: FnOnce() -> R + Send + 'static,
        R: 'static;

    /// See [`remove_non_send_resource`].
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_command_non_send::CommandsExt;
    /// #
    /// struct MyNonSend(*const u8);
    ///
    /// fn remove_my_non_send(mut commands: Commands) {
    ///     commands.remove_non_send_resource::<MyNonSend>();
    /// }
    /// #
    /// # App::new()
    /// #     .insert_non_send_resource(MyNonSend(std::ptr::null()))
    /// #     .add_systems(Startup, (remove_my_non_send, check).chain())
    /// #     .run();
    /// #
    /// # fn check(my_non_send: Option<NonSend<MyNonSend>>) {
    /// #     assert!(my_non_send.is_none());
    /// # }
    /// ```
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

// Sealed trait used to prevent others from implementing `CommandsExt`.
mod private {
    use bevy_ecs::system::Commands;

    pub trait Sealed {}

    impl Sealed for Commands<'_, '_> {}
}
