# Sust

Sust is a modern, role-based murder mystery game built using Rust and React.

## Prerequisites

- [Rust (&geq;1.63)](https://rustup.rs/)
- [Cargo Watch](https://crates.io/crates/cargo-watch)
- [Node.js (&geq;16.17.0)](https://nodejs.org/en/)

## Local Development

To run the application, run `npm start`.
This will use `react-scripts` to serve the front-end code on https://localhost:3000.
The web server is also launched on https://localhost:3001, but only receives traffic that cannot be handled by the front end (e.g. https://localhost:3001/login).
When deployed, the web server will handle all traffic to the application on a single port.

Local development also features automatic code reloading when changes are detected, using `cargo-watch` for the front-end code and `react-scripts` for the back-end code.

## Deployment

To build the entire application, run `npm run build`.
This will create a binary located at `server/target/release/sust-server` that launches the web server when executed.
It will also package the front-end code into `build/`, which is then served by the web server.
