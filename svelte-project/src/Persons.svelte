<script lang="ts">
    export let names;
</script>

<div class="persons">
    <h5>Rust state:</h5>
    {#each names as name, i (name)}
        <div> {name.id} {name.first_name}</div>
    {/each}
</div>
