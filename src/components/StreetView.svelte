<script lang="ts">
    import { Loader } from "@googlemaps/js-api-loader";

    let props: { coords: { lat: number; lng: number } } = $props();

    const loader = new Loader({
        // don't get excited, this api key is from the documentation :)
        apiKey: "AIzaSyB41DRUbKWJHPxaFjMAwdrzWzbVKartNGg",
        version: "weekly",
    });

    let map: google.maps.StreetViewPanorama;

    $effect(() => {
        (async () => {
            const { StreetViewPanorama } =
                await loader.importLibrary("streetView");

            map = new StreetViewPanorama(
                document.getElementById("view") as HTMLElement,
                {
                    position: props.coords,
                    addressControl: false,
                    linksControl: false,
                    showRoadLabels: false,
                    visible: true,
                },
            );
        })();
    });

    $effect(() => {
        if (props.coords && map) {
            map.setPosition(props.coords);
        }
    });
</script>

<div id="view" style="width: 50%; height: 500px;"></div>
