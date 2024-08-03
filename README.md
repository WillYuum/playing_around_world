# Playing Around World

A game idea that's not finished but interested in having over 1000 enemies attacking the center base.
Let's see how this goes

Using bevy game engine with ECS style programming to achieve this insance goal

![image](https://github.com/WillYuum/playing_around_world/assets/33324037/5441cd70-6f53-4f91-86fc-2901ae37440a)

## Run

1. Make sure to have Rust installed to latest version.
2. Run `cargo run`. 

## Build

### Windows:
``` ./export_windows_86_64.sh ```

### Web

1. Install wasm32   
```rustup target install wasm32-unknown-unknown```

2. Make sure you have wasm-bindgen-cli   
``` $ cargo install -f wasm-bindgen-cli```

3. Run web build shell script   
``` ./export_web_build.sh ```