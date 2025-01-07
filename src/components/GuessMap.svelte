<script lang="ts">
    import markerIconUrl from "leaflet/dist/images/marker-icon.png";
    import markerIconRetinaUrl from "leaflet/dist/images/marker-icon-2x.png";
    import markerShadowUrl from "leaflet/dist/images/marker-shadow.png";

    import "leaflet/dist/leaflet.css";
    import * as leaflet from "leaflet";
    import type { LatLng } from "../utils";
    import * as L from "leaflet.geodesic";

    leaflet.Icon.Default.prototype.options.iconUrl = markerIconUrl;
    leaflet.Icon.Default.prototype.options.iconRetinaUrl = markerIconRetinaUrl;
    leaflet.Icon.Default.prototype.options.shadowUrl = markerShadowUrl;
    leaflet.Icon.Default.imagePath = "";

    let {
        query = $bindable(),
        result,
        showResult,
    }: {
        query: LatLng | null;
        result: LatLng | null;
        showResult: boolean;
    } = $props();

    const setDefaultView = () => map?.setView([27, 4], 2);

    let map: leaflet.Map | null = $state(null);
    let resultLine: L.GeodesicLine | null = null;

    $effect(() => {
        if (!showResult) {
            setDefaultView();
        } else if (showResult && query && result) {
            map?.flyToBounds(
                [
                    [query.lat, query.lng],
                    [result.lat, result.lng],
                ],
                { padding: [10, 10] },
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
        setDefaultView();
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

<div style="width: 50%; position: relative;">
    {#if !query}
        <div
            style="position: absolute; z-index: 999; margin: 1rem 0 0 1rem; background: white; padding: 0.5rem"
        >
            <i>click to make a guess</i>
        </div>
    {/if}
    <div use:mapAction style="width: 100%; height: 100%;"></div>
</div>
