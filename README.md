# chorddb

A WEB tablature viewer

## Dependencies

In order to run ChordDB you need Rust and a JS runtime

### Rust

You can install [Rust](https://www.rust-lang.org/) following [their instructions](https://www.rust-lang.org/tools/install)

### JS runtime

This was tested using [Bun](https://bun.sh/) and [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm). The command line examples will use `bun`/`bunx`, but the translate directly to `npm`/`npx`.

## Running

ChordDB is composed of two parts, the Rust BE and the JS FE app.

You can run the backend using

```
cargo run
```

The frontend app resides in the [frontend] directory. You can run it with

```
cd frontend
bun install && bun run dev
```

You should see the app running in http://localhost:5173 after that
