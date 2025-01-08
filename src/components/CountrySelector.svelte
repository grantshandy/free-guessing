<script lang="ts">
    import type { ChangeEventHandler } from "svelte/elements";
    import countries from "../countries.json";
    import type { CountryCode } from "../utils";

    let { countryCode = $bindable(), onchange }: { countryCode: CountryCode | null, onchange: ChangeEventHandler<HTMLSelectElement> } = $props();

    const regionNames = new Intl.DisplayNames( ['en'], {type: 'region'} );
</script>

<select bind:value={countryCode} {onchange} class="select select-bordered select-sm">
    <option selected value={null}>All Countries</option>
    {#each Object.keys(countries).sort() as c}
        <option value={c}>{regionNames.of(c)}</option>
    {/each}
</select>
