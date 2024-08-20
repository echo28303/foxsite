# Foxtrot Blockchain Website

This is the official website for the Foxtrot blockchain, showcasing the power of Rust, WebAssembly, and Yew.

## Project Structure

- `frontend`: Contains the Yew-based frontend of the website.
- `backend`: Contains the Rust-based backend with GraphQL API.

## How to Run

1. Install Rust and add the WebAssembly target:

rustup target add wasm32-unknown-unknown

2. Install Trunk:

cargo install --locked trunk

3. Navigate to the `frontend` directory and run the site:

trunk build --release

trunk serve

4. Navigate to the `backend` directory and start the server:

cargo run


