<script lang="ts">
    export let wasm;

    import {ProgressCircular, MaterialApp, TextField, Button, Col} from 'svelte-materialify';
    import Persons from "./Persons.svelte";

    wasm.get_state().names.map(name => console.log("ID: ", name.id, "; FIRST_NAME: ", name.first_name));

    let name = "";
    let state = wasm.get_state();
    let progress = false;

    const setName = (newName) => {
        name = newName;
    }

    const setState = (newState) => {
        state = newState;
    }

    const setProgress = (newProgress) => {
        progress = newProgress;
    }

    const dispatch = (actionType, action) => {
        setProgress(true);
        wasm.dispatch(actionType, action).then((arg0) => {
            // console.log("arg0: " + arg0);
            setState(wasm.get_state());
            setProgress(false);
            state.names.map(name => console.log("ID2: ", name.id, "; FIRST_NAME2: ", name.first_name));
        }, (err) => {
            console.log("ERROR " + err);
        });
    }

    const handleChange = (e) => {
        setName(e.target.value);
    }

    const handleAddName = () => {
        dispatch("AddName", {first_name: name});
    }
</script>

<MaterialApp>
    <Col cols={2}>
        {#if progress}
            <ProgressCircular indeterminate color="red"/>
        {/if}
        <br/>
        <br/>
        <TextField bind:value={name} clearable color="red">
            Name
        </TextField>
        <Button class="red white-text" on:click={() => handleAddName()}>
            Add name
        </Button>
        <br/>
        <br/>
        <Persons bind:state/>
    </Col>
</MaterialApp>


