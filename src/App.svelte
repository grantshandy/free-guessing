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
      newGuess();
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

<main class="space-y-3 p-3">
  <h1 class="text-2xl font-semibold">Free Guessing</h1>

  <p>
    Guess the location of random street views across the world, inspired by <a
      href="https://www.geoguessr.com/">geoguessr</a
    >.
  </p>

  {#if loading}
    <h2>Loading Database...</h2>
  {:else}
    <div class="flow-root">
      <CountrySelector bind:countryCode onchange={newGuess} />
      {#if !resultScreen}
        <button class="btn btn-sm" onclick={newGuess}>Skip</button>
      {/if}

      <div class="float-right flex items-center space-x-3">
        {#if resultScreen}
          <span>Score:</span>
          <progress value={score} max="100" class="progress hidden md:flex w-64"
          ></progress>
          <span>{score?.toFixed(2)}%</span>
          <button onclick={newGuess} class="btn btn-sm btn-primary"
            >Next!</button
          >
        {:else}
          <button
            disabled={!query}
            onclick={makeGuess}
            class="btn btn-sm btn-primary">Guess</button
          >
        {/if}
      </div>
    </div>

    {#if target}
      <div class="grid grid-cols-1 md:grid-cols-2 w-full h-[70vh] md:h-[60vh]">
        <StreetView coords={target} />
        <GuessMap bind:query result={target} showResult={resultScreen} {countryCode} />
      </div>
    {/if}
  {/if}

  <p class="text-center">
    <a href="https://buymeacoffee.com/granthandy">Donate</a> &#183;
    <a href="https://github.com/grantshandy/free-guessing">Source</a>
  </p>
</main>
