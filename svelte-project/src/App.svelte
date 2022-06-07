<script lang="ts">
    export let getState;
    export let dispatch;

    import {ProgressCircular, MaterialApp, TextField, Button, Col} from 'svelte-materialify';
    import Persons from "./Persons.svelte";

    getState.names.map(name => console.log("ID: ", name.id, "; FIRST_NAME: ", name.first_name));

    let name = "";
    let state = getState;
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

    const disp = (actionType, action) => {
        setProgress(true);
        dispatch(actionType, action).then((arg0) => {
            // console.log("arg0 " + arg0);
            setState(getState);
            setProgress(false);
        }, (err) => {
            console.log("ERROR " + err);
        });
    }

    const handleChange = (e) => {
        setName(e.target.value);
    }

    const handleAddName = () => {
        // const { dispatch } = props;
        disp("AddName", {first_name: name});
        // setPersons(names);
    }
</script>

<MaterialApp>
    <Col cols={2}>
        {#if progress}
            <ProgressCircular indeterminate color="primary"/>
        {/if}
        <br/>
        <br/>
        <TextField bind:value={name} clearable>
            Name
        </TextField>
        <Button class="primary-color" on:click={() => handleAddName()}>
            Add name
        </Button>
        <br/>
        <br/>
        <Persons state={state}/>
    </Col>
</MaterialApp>


