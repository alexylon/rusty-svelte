<script lang="ts">
    export let state;
</script>

<div class="persons">
    <h5>Rust state:</h5>
    {#each state.names as name, i (name)}
        <div> {name.id} {name.first_name}</div>
    {/each}
</div>
