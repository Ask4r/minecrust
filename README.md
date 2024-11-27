# Minecrust

A simle Minecraft clone written in `bevy` ([data-driven game engine built in Rust](https://github.com/bevyengine/bevy)) 
and ported to SPA web-app written in `React`

## Goals

This project is an example of how to create package with rust-generated WebAssembly, and further use it with any 
`Node`-compatible JavaScript framefork.

It might be a good refence to how to deal with WebAssembly's time-costly compilations and difficult 
client-side debugging by splitting your development cycles.
Project allows to run **minecrust** natively or compile it with `wasm-pack`
and run it in browser with provided frontend.

`Vite` and `React` are used as minimal subset of popular web-technologies. It is perfect to show ease of
integration and near-native perfomance of tools and libraries written in compiled languages.

## Requirements

* **rust** - modern and robust language with good wasm support
* **wasm-pack** - tool for building and working with rust-generated WebAssembly
* **node/npm** - JavaScript runtime and package manager

## Installing dependencies
Rust `cargo` package manager will fetch all its dependencies automatically during your first build.
You only need to install node dependencies one of which is created **wasm** package.
To create it firstly run `wasm-pack` build (script available with `npm run`). Then install remaining dependencies with npm. 
```sh
# will create rust npm package with `wasm-pack` at `world/pkg`
# see `Building and running` section in README.md 
npm run build:wasm-release 

# install `pkg` and remaining dependencies
npm install
```

## Building and running

### WARNING
`rust` build process is very costly especially when compiling to **wasm** and especially during **dev** build.
It may take up to 6.5GB of RAM, all available CPU cores and up to 5min of compile and build time. 
And your first build must be even bigger, since it will compile entire `bevy` game engine.
Make sure you have closed unused apps and windows to free CPU and memory for build process.

All scripts for building and serving are included. You can find them in `package.json` scripts.

### Release build
Building local server consists of 3 steps:
1. Compiling and packing **wasm**
```sh
npm run build:wasm-release
```
2. Copy **minecrust** assets so they are statically served by `Vite`
```sh
npm run load:wasm-assets
```
3. Build `React` app
```sh
npm run build
```
Now you will have you build in `dist` dir. Deploy it statically as-is or run locally with 
```sh
# vite's preview...
npm run preview
# ...or static serve
npx serve dist
```

### Dev build
As it was mentioned, **dev** wasm builds are the most costly and can really **crash** your computer.
Close any opened apps, your browser and unused processes and be ready to terminate process if necessary.
If your machine cannot run such expensive tasks, try running `release` build process for rust. 
It takes longer but somehow less greedy in memory.

Steps are similar:
1. Compiling and packing **wasm**
```sh
npm run build:wasm
# try release build for poor-memory (8GB or less) machines
npm run build:wasm-release
```
2. Copy **minecrust** assets so they are statically served by `Vite`
```sh
npm run load:wasm-assets
```
3. Run local `Vite` server
```sh
npm run dev
```
Note: vite is configured to run in MPA mode to preserve 404 for non-existing files.
Bevy uses internal asset server to check if .meta files exist and vite's success responses
wrappings during dev runs confuse it, causing errors.

### Native rust build
**wasm** building pipelines are long, memory-consuming and debugging builds on client is 
only possible via log messages. Porting packages to web is easy with existing tools, but developing them is 
slow, tough and inconvinient. Purpose of this project is to aknowledge this issues and show how to deal with them.

To run native build cd into `world` directory and run **minecrust** with `cargo`. 
If you run this after build of the whole project - don't worry, rust's native builds are much less
greedy and more developer-friendly.
```sh
cd world
# cargo will fetch all dependencies and
# build entire bevy game engine
# so first build will take longer
cargo build
cargo run
```
Arter running this a new window should pop-up and a game should start.
As you can see, building is still long, but tolerable. 
Since `bevy` supports WebAssebly builds, you should rely on its cross-platform possibilities
and develop logic and fetures natively. 
Then build and test implementation for web and repeat the cycle for new features to stay productive.

## Conclusion

During this project I suffered long time compilations, lack of RAM and build crashes.
I had to close my editor and browser to free up memory. And then rerun builds just to put new 
log line trying to fix the game. Then open editor and browser again and repeat... 
But if I didn't close either of them, the build would go out of RAM and would take up disk space, 
crashing my machine, unable to even move my mouse and stay like that for 28min.

The project was dropped for a while, but rethinking the way of developing game features and its frontend separately
opened new possibilities for relatively fast debugging and client-side testing. 
It became more clear which were problems with logic and which platform-related, making them easier to solve.
When put like this it might be obvious, but trying to speed up development by building
to non-related target is not something that comes to mind.

Web-apps are different in many senses and compilation is one of these. 
As it appears, it's less convinient to test and run browser library at browser directly,
rather than making relatively quick native build, just to confirm your hypotheses
and repeatedly try new ones after.

Everyone knows that different platforms are different and some things might not work as expected.
But in this particular situation the turnarounds appeared to be cheaper.
Spending time fixing wrong things was more productive than trying to build right things at once.

Wasm cross-compile tools should be used for existing and tested software.
As there are no languages/libraries/tools for building WebAssembly only,
they all support native pipelines as they support wasm.
Keep that in mind when you create new WebAssembly project, and if you have one
but dont't support native build, consider creating one as spent time will surely
pay-off.

Engineering is not just debugging, it's the whole way of creation.
