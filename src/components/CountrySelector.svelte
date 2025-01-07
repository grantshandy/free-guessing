<script lang="ts">
    import countries from "../countries.json";

    let { countryCode = $bindable() }: { countryCode: string | null } = $props();

    const regionNames = new Intl.DisplayNames( ['en'], {type: 'region'} );
</script>

<select bind:value={countryCode}>
    <option selected value={null}>All Countries</option>
    {#each countries.codes.sort() as c}
        <option value={c}>{regionNames.of(c)}</option>
    {/each}
</select>
