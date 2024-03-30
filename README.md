# `bevy_command_non_send`

This is a small utility library for manipulating non-`Send` resources using [`Commands`] in [Bevy].

[`Commands`]: https://docs.rs/bevy/0.13.1/bevy/ecs/system/struct.Commands.html
[Bevy]: https://bevyengine.org

## Installation

This library is not added on crates.io, so you will need to depend on the Git repository directly.

```bash
$ cargo add --git https://github.com/BD103/bevy_command_non_send bevy_command_non_send
```

Alternatively, you can add it directly to `Cargo.toml`:

```toml
[dependencies]
bevy_command_non_send = { version = "0.x", git = "https://github.com/BD103/bevy_command_non_send" }
```

## Usage

Please see the [docs] for the API reference. You most likely want to imports the `CommandsExt` trait, which adds several methods on top of `Commands`:

[docs]: https://bd103.github.io/bevy_command_non_send/bevy_command_non_send/

```rust
use bevy::prelude::*;
use bevy_command_non_send::CommandsExt;

struct MyNonSend(*const u8);

fn create_my_non_send(mut commands: Commands) {
    // `insert_non_send_resource` is a method imported with `CommandsExt`.
    commands.insert_non_send_resource(|| {
        MyNonSend(std::ptr::null())
    });
}

App::new()
    .add_systems(Startup, create_my_non_send)
    .run();
```

## Bevy Compatibility

|Bevy|`bevy_command_non_send`|
|-|-|
|0.13|0.1|

## License

`bevy_command_non_send` is dual-licensed under either

- [MIT License] (<http://opensource.org/licenses/MIT>)
- [Apache 2.0 License] (<http://www.apache.org/licenses/LICENSE-2.0>)


at your option.

[MIT License]: https://github.com/BD103/bevy_command_non_send/blob/main/LICENSE-MIT
[Apache 2.0 License]: https://github.com/BD103/bevy_command_non_send/blob/main/LICENSE-APACHE

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
