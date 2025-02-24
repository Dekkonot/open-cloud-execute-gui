# Open Cloud Execute GUI

A Tauri-based app for running scripts using Roblox's Open Cloud Luau execution API. Hopefully self-explanatory once you give it a try.

![](app.png)

You do not need to specify a place version; it will run on latest if none is given.

## Building

Install Rust and NPM. Run `npm install` in the repo and then run `npm run tauri build`. After Vite and Cargo finish building, the bundled program will be in `src-tauri/target/release`.
