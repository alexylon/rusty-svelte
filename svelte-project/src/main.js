import App from './App.svelte';
import rustyWasm from '../../rust-project/Cargo.toml';

const init = async () => {
    const wasm = await rustyWasm();

    console.log('init WASM');
    // rustProject.get_state().names.map(name => console.log("id: ", name.id, "; first_name: ", name.first_name));

    const app = new App({
        target: document.body,
        props: {
            // https://svelte.dev/docs#Creating_a_component
            wasm: wasm
        }
    });

};

init();
