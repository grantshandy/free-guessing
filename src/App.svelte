<script lang="ts">
  // @ts-ignore
  import searchIcon from "svelte-awesome/icons/search";

  import StreetView from "./components/StreetView.svelte";
  import CountrySelector from "./components/CountrySelector.svelte";
  import GuessMap from "./components/GuessMap.svelte";

  import {
    loadDatabase,
    calculateScore,
    type GetRandomLocation,
    type LatLng,
    type CountryCode,
  } from "./utils";
  import JSConfetti from "js-confetti";
  import { fade } from "svelte/transition";
  import { Icon } from "svelte-awesome";

  let guessDialog: HTMLDialogElement;

  let loading = $state(true);
  let getRandomLocation: GetRandomLocation;

  let countryCode: CountryCode | null = $state(null);
  let query: LatLng | null = $state(null);
  let target: LatLng | null = $state(null);
  let resultScreen: boolean = $state(false);

  let score: number | null = $derived(
    query && target ? calculateScore(query, target, countryCode) : null,
  );

  $effect(() => {
    loadDatabase().then((f) => {
      getRandomLocation = f;
      loading = false;
      newGuess();
    });
  });

  const jsConfetti = new JSConfetti();

  const newGuess = () => {
    query = null;
    resultScreen = false;
    target = getRandomLocation(countryCode);
  };

  const makeGuess = () => {
    resultScreen = true;
    if (score && score > 80) {
      jsConfetti.addConfetti({
        confettiRadius: 6,
        confettiNumber: 1000,
      });
    }
  };
</script>

<main
  class="mx-auto md:w-2/3 xl:w-1/2 h-[100vh] flex flex-col space-y-6 px-3 py-8"
>
  <div class="space-y-1">
    <h1 class="text-2xl font-semibold">Free Guessing</h1>

    <p>
      Guess the location of random street views across the world, inspired by <a
        href="https://www.geoguessr.com/">geoguessr</a
      >.
    </p>
  </div>

  {#if loading}
    <h2>Loading Page...</h2>
  {/if}

  {#if target}
    <div class="grow flex flex-col space-y-2">
      <div class="w-full flow-root space-x-2">
        <CountrySelector bind:countryCode onchange={newGuess} />
        <button class="btn btn-sm btn-ghost float-right" onclick={newGuess}
          >Skip</button
        >
      </div>

      <div
        class="grow relative rounded-box overflow-hidden border border-base-300"
      >
        <button
          class="absolute top-2 left-2 shadow-md btn btn-circle z-[999]"
          aria-label="guess box"
          onclick={() => guessDialog.showModal()}
        >
          <Icon data={searchIcon} scale={1.5} />
        </button>
        <StreetView coords={target} />
      </div>
    </div>
  {/if}

  <hr class="bg-neutral-content h-px border-0" />

  <p class="text-center">
    <a href="https://buymeacoffee.com/granthandy">Donate</a> &#8729;
    <a href="https://github.com/grantshandy/free-guessing">Source</a>
  </p>
</main>

<dialog class="modal" bind:this={guessDialog}>
  <div
    class="modal-box rounded-box overflow-visible relative flex flex-col w-full mx-4 h-[70vh] border border-neutral-content p-0"
  >
    {#if resultScreen && score != null}
      <span
        transition:fade={{ duration: 400 }}
        class="absolute top-0 right-0 rounded-l-full sm:rounded-r-full shadow-lg py-2 px-4 bg-base-200 font-semibold border text-lg z-[999] translate sm:translate-x-1/4 -translate-y-1/3"
      >
        Score: {score.toFixed(2)}%
      </span>
    {/if}

    <div class="grow overflow-hidden rounded-t-box">
      <GuessMap
        bind:query
        result={target}
        showResult={resultScreen}
        {countryCode}
      />
    </div>

    <button
      disabled={!query}
      onclick={() => {
        if (resultScreen) {
          newGuess();
          guessDialog.close();
        } else {
          makeGuess();
        }
      }}
      class="btn btn-sm btn-primary rounded-t-none rounded-b-box"
    >
      {#if !query}
        Click to Make a Guess
      {:else if !resultScreen}
        Submit
      {:else}
        Next
      {/if}
    </button>
  </div>
  <form
    method="dialog"
    class="modal-backdrop"
    onsubmit={() => {
      if (resultScreen) newGuess();
    }}
  >
    <button>close</button>
  </form>
</dialog>
