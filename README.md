# Pulsar Migrator Issue Bot

This app is written to help migrate old atom packages archived with [AtomPackagesArchive] by [confused-Techie] to the new [Pulsar backend].

I did also take my leisure to play around with libraries and things that I have not used before, which is partly why progress is a bit slow. :3

## requirements

- [rust], at least version 1.62.0

## build

- `cargo build --release`
- artifact will be in `target/release/pulsar-migrator-issue-bot`. It is a standalone binary, the rest of the `target` folder can be deleted if you wish to save storage space

## usage

- See `.env.example` for environment variables that need to be set. You don't need to use a `.env` file if you don't want to of course, but it is supported.
- `pulsar-migrator-issue-bot --help` can give some help too

General steps to run to get it up and running:

- `pulsar-migrator-issue-bot read-package-data ...[files to import from]` imports data from these files. Each file should have one package. For example, if using [confused-Techie's migrated package data], you should pass all the packages in `out/packages`, not the `package_pointer.json` file.
- more stuff incoming soon&trade;!
- the state of the packages are stored in `state.ron` in a pretty formatted way. You can edit it if you know what you are doing.

[AtomPackagesArchive]: https://github.com/confused-Techie/AtomPackagesArchive
[confused-Techie]: https://github.com/confused-Techie
[confused-Techie's migrated package data]: https://github.com/confused-Techie/atom-package-collection
[Pulsar backend]: https://github.com/confused-Techie/atom-community-server-backend-JS
[rust]: https://www.rust-lang.org
