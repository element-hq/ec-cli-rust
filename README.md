# Introduction

This tool currently supports a single mode where it simulates Element Call
users joining the call. It could be used to create bot participants in any
call.

# Usage

Install Rust (if not already installed) with:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

Then you can run `cargo run --release` to see all available options.

Example of running:

```
cargo run --release -- https://pr805--element-call.netlify.app/room/#test123:call.ems.host 3 --headless
```

This will launch 3 bot users that would use the specified URL to join the call.

# How It Works

This tool tries to detect a local installation of Google Chrome (or install one
if it does not exist). The Chrome browser is launched and being controlled by
the tool over the DevTools Protocol, i.e. the tool simulates clicking in the
interface.

Note that it may seem like it runs slow (a lot of time passes between each bot
is run). It is intentional to workaround possible rate limiting issues. Since
Element Call relies upon Matrix infrastructure, a new user is registered each
time we're attempting to join a call. Registering a user or logging-in with a
user are both subjects to rate limiting.
