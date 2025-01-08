<script lang="ts">
    import markerIconUrl from "leaflet/dist/images/marker-icon.png";
    import markerIconRetinaUrl from "leaflet/dist/images/marker-icon-2x.png";
    import markerShadowUrl from "leaflet/dist/images/marker-shadow.png";
    leaflet.Icon.Default.prototype.options.iconUrl = markerIconUrl;
    leaflet.Icon.Default.prototype.options.iconRetinaUrl = markerIconRetinaUrl;
    leaflet.Icon.Default.prototype.options.shadowUrl = markerShadowUrl;
    leaflet.Icon.Default.imagePath = "";

    import "leaflet/dist/leaflet.css";
    import * as leaflet from "leaflet";
    import type { LatLng } from "../utils";
    import * as L from "leaflet.geodesic";

    import boundingBoxes from "../countries.json";
    type BoundingBoxes = typeof boundingBoxes;
    type CountryKey = keyof BoundingBoxes;

    let {
        query = $bindable(),
        result,
        showResult,
        countryCode,
    }: {
        query: LatLng | null;
        result: LatLng | null;
        showResult: boolean;
        countryCode: string | null;
    } = $props();

    const setWorldwideView = () => map?.setView([27, 4], 2);

    let map: leaflet.Map | null = $state(null);
    let resultLine: L.GeodesicLine | null = null;

    $effect(() => {
        if (!showResult) {
            if (countryCode && countryCode in boundingBoxes) {
                const bounds = boundingBoxes[countryCode as CountryKey];

                map?.flyToBounds(
                    [
                        [bounds[1], bounds[0]],
                        [bounds[3], bounds[2]],
                    ],
                    { padding: [10, 10] },
                );
            } else {
                setWorldwideView();
            }
        } else if (showResult && query && result) {
            map?.flyToBounds(
                [
                    [query.lat, query.lng],
                    [result.lat, result.lng],
                ],
                { padding: [20, 20] },
            );
        }
    });

    const queryMarker = leaflet.marker({ lat: 0, lng: 0 });
    $effect(() => {
        if (!map) return;
        if (!query) queryMarker.remove();
    });

    const targetMarker = leaflet.marker({ lat: 0, lng: 0 });
    $effect(() => {
        if (!map) return;
        if (query && result && showResult) {
            resultLine = new L.GeodesicLine([query, result], {
                weight: 2,
                steps: 10,
            }).addTo(map);
            targetMarker.setLatLng(result).addTo(map);
        } else {
            resultLine?.remove();
            targetMarker.remove();
        }
    });

    const mapAction = (cont: HTMLDivElement) => {
        map = leaflet.map(cont, {
            preferCanvas: false,
            minZoom: 2,
            zoomControl: false,
        });
        setWorldwideView();
        map.setMaxBounds(new leaflet.LatLngBounds([-150, -300], [150, 400]));
        leaflet
            .tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
                noWrap: true,
                attribution:
                    '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>',
            })
            .addTo(map);

        map.on("click", (ev: { latlng: LatLng }) => {
            if (showResult) return;

            if (map && !map.hasLayer(queryMarker)) {
                queryMarker.addTo(map);
            }
            query = ev.latlng;
            queryMarker.setLatLng(ev.latlng);
        });

        return {
            destroy: () => map?.remove(),
        };
    };
</script>

<svelte:window on:resize={() => map?.invalidateSize()} />

<div class="w-full h-full relative">
    {#if !query}
        <p
            class="absolute z-[999] ml-1 mt-1 bg-base-200 py-0.5 px-2 italic rounded-md shadow-md"
        >
            click to make a guess
        </p>
    {/if}
    <div
        use:mapAction
        class="w-full h-full"
        style="cursor: pointer !important;"
    ></div>
</div>
