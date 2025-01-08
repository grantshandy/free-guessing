import wasm from "sql.js/dist/sql-wasm.wasm?url";
import dbUrl from "/locations.sqlite?url";
import * as initSqlJs from "sql.js/dist/sql-wasm";
import countries from "./countries.json";
import type { IconData } from "svelte-awesome/components/Icon.svelte";
import { DivIcon, divIcon } from "leaflet";

export type GetRandomLocation = (countryCode: string | null) => LatLng;
export type LatLng = { lat: number; lng: number };

export type CountryBoundingBoxes = typeof countries;
export type CountryCode = keyof CountryBoundingBoxes;

// Returns a function that can generate random google street view locations from the database
export const loadDatabase = async (): Promise<GetRandomLocation> => {
    const sql = await initSqlJs.default({ locateFile: () => wasm });

    const dbBytes = await fetch(dbUrl)
        .then((r) => r.blob())
        .then((b) => b.arrayBuffer());

    const db = new sql.Database(new Uint8Array(dbBytes));

    return (countryCode: string | null) => {
        const sql = countryCode
            ? `SELECT latlng
                FROM locations
                JOIN country_codes
                ON locations.country_id = country_codes.country_id
                WHERE country_codes.country_code = "${countryCode}"
                ORDER BY RANDOM()
                LIMIT 1`
            : `SELECT latlng
                FROM locations
                ORDER BY RANDOM()
                LIMIT 1`;

        let latlng = new DataView((db.exec(sql)[0].values[0][0] as Uint8Array).buffer);

        return {
            lat: latlng.getFloat32(0, true),
            lng: latlng.getFloat32(4, true)
        };
    };
}

export const renderIcon = (icon: IconData): DivIcon => {
    // @ts-ignore
    const html = `<div class="p-1 rounded-full bg-primary fill-primary-content shadow-lg w-full h-full">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-full w-full" viewBox="0 0 ${icon.width} ${icon.height}">
                        ${(icon.paths ?? []).map((path) => `<path d="${path.d}"/>`)}
                    </svg>
                </div>`;

    const size = 25;

    return divIcon({ html, className: "", iconSize: [size, size], shadowSize: [size, size] });
}

// Generates a score from 0 to 100 for your guess. Closer points, higher scores.
export const calculateScore = (pointA: LatLng, pointB: LatLng, countryCode: CountryCode | null): number => {
    const dist = calculateDistanceKm(pointA, pointB);
    let maxDistKm: number;

    if (countryCode) {
        const bounds = countries[countryCode];
        maxDistKm = calculateDistanceKm({ lat: bounds[1], lng: bounds[0] }, { lat: bounds[3], lng: bounds[2] });
    } else {
        maxDistKm = 20_000;
    }

    // TODO: tweak?
    const factor = (maxDistKm * maxDistKm) / 20_000 + 50;
    return 100 * Math.exp(-dist / factor);
};

const calculateDistanceKm = (pointA: LatLng, pointB: LatLng): number => {
    const deg2rad = (deg: number) => deg * (Math.PI / 180);

    var dLat = deg2rad(pointB.lat - pointA.lat);
    var dLng = deg2rad(pointB.lng - pointA.lng);

    var a =
        Math.sin(dLat / 2) * Math.sin(dLat / 2) +
        Math.cos(deg2rad(pointA.lat)) * Math.cos(deg2rad(pointB.lat)) *
        Math.sin(dLng / 2) * Math.sin(dLng / 2)
        ;
    var c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

    return 6371 * c; // * radius of the earth in km
}
