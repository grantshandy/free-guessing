<script lang="ts">
  import StreetView from "./components/StreetView.svelte";
  import CountrySelector from "./components/CountrySelector.svelte";
  import GuessMap from "./components/GuessMap.svelte";

  import {
    loadDatabase,
    calculateScore,
    type GetRandomLocation,
    type LatLng,
  } from "./utils";

  let loading = $state(true);
  let getRandomLocation: GetRandomLocation;

  let countryCode: string | null = $state(null);
  let query: LatLng | null = $state(null);
  let target: LatLng | null = $state(null);
  let resultScreen: boolean = $state(false);

  let score: number | null = $derived(
    resultScreen && query && target
      ? calculateScore(query, target, countryCode)
      : null,
  );

  $effect(() => {
    loadDatabase().then((f) => {
      getRandomLocation = f;
      loading = false;
    });
  });

  const newGuess = () => {
    query = null;
    resultScreen = false;
    target = getRandomLocation(countryCode);
  };

  const makeGuess = () => {
    resultScreen = true;
  };
</script>

<main>
  <h2>Free Guessing</h2>

  {#if loading}
    <h3>Loading Database...</h3>
  {:else}
    <hr />
    <div>
      <CountrySelector bind:countryCode />
      <button onclick={newGuess}>Random Location</button>

      <div style="float: right; display: flex;">
        {#if !resultScreen && target}
          <button disabled={!query || resultScreen} onclick={makeGuess}
            >Guess</button
          >
        {/if}

        {#if resultScreen && score != null}
          <progress value={score} max="100"></progress>
          <span style="margin-left: 0.5rem;">{score.toFixed(2)}%</span>
          <button style="margin-left: 1rem;" onclick={newGuess}>Next!</button>
        {/if}
      </div>
    </div>
    <hr />

    {#if target}
      <div style="display: flex;">
        <StreetView coords={target} />
        <GuessMap bind:query result={target} showResult={resultScreen} />
      </div>
      <hr />
    {/if}
  {/if}
</main>
