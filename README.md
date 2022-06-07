# rusty-svelte

rusty-svelte is an example project that how WebAssembly and Svelte could work
together.

This example project uses:
- [Svelte](https://svelte.dev/) as frontend "Framework".
- [RollupJs](https://rollupjs.org/guide/en/) as module bundler.
- [wasm-rollup-plugin](https://github.com/wasm-tool/rollup-plugin-rust) the perfect Helper to integrate your Rust code into your js env.
- [Rust](https://www.rust-lang.org/)
- [wasm-pack](https://github.com/rustwasm/wasm-pack) awesome Rust WebAssembly Generator.

Is for: The main focus of this project is to make an integration example of WebAssembly (Rust) and Svelte.
Is not for: This project is neither a coding example from Rust nor from Svelte.

## Project structure:
```
rusty-svelte
├── rust-project
└── svelte-project
```

## Setup wasm
1. Install the [rollup-plugin-rust](https://github.com/wasm-tool/rollup-plugin-rust) plugin.
2. Setup the plugin in your `rollup.config.js`
   ```js
   // ...
   import rust from "@wasm-tool/rollup-plugin-rust";

   export default [{
     // ...

     plugins: [
       // ...

       // Add the configuration for your wasm-tool plugins
       // The generated .wasm file is placed in the /build/ folder.
       // To tell the server where to fetch the .wasm file you have to specify
       // the path otherwise you get a 404 error (.wasm file not found).
       rust({
        verbose: true,
        serverPath: "/build/"
      }),
     ]
   }]
   ```
3. Access your wasm `get_state`and `dispatch` functions in your Svelte js code.

   ```js
   // svelte-app/src/main.js
   import App from './App.svelte';
   // Load the .toml file of your Rust project.
   // The wasm-plugin runs `wasm-pack build` and cpoies the output into
   // `svelte-app/target` directory.
   // The `.wasm` file is located in the `svelte-app/public/build` dir.
   import wasm from '../../rust-project/Cargo.toml';

   // WebAssembly files must be loaded async.
   const init = async () => {
       const rustProject = await wasm();

       const app = new App({
           target: document.body,
           props: {
             // https://svelte.dev/docs#Creating_a_component
             getState: rustProject.get_state(),
             dispatch: rustProject.dispatch
           }
       });
   };

   init();
   ```
4. Start the server `npm run dev`.
   The output should look something like this:
   ```bash
   Your application is ready~! 🚀

   - Local:      http://localhost:5000
 
   ────────────────── LOGS ──────────────────

   [23:02:30] 200 ─ 5.79ms ─ /
   [23:02:30] 200 ─ 1.51ms ─ /global.css
   [23:02:30] 200 ─ 2.81ms ─ /build/bundle.css
   [23:02:30] 200 ─ 3.40ms ─ /build/bundle.js
   [23:02:31] 200 ─ 2.04ms ─ /build/rusty-svelte.wasm <-- The defined build path in your rollup.config.js file.
   [23:02:31] 200 ─ 4.86ms ─ /build/bundle.css.map
   [23:02:31] 200 ─ 7.84ms ─ /build/bundle.js.map
   [23:02:31] 200 ─ 1.20ms ─ /favicon.png
   ```
