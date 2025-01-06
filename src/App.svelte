<script lang="ts">
  let a: number = $state(0);
</script>

<main>
  <h2>Geoguessing</h2>
  <p><button onclick={() => a--}>-</button> {a} <button onclick={() => a++}>+</button></p>
</main>
