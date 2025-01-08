import wasm from "sql.js/dist/sql-wasm.wasm?url";
import dbUrl from "/locations.sqlite?url";
import * as initSqlJs from "sql.js/dist/sql-wasm";

export type GetRandomLocation = (countryCode: string | null) => LatLng;
export type LatLng = { lat: number; lng: number };

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

// Generates a score from 0 to 100 for your guess. Closer points, higher scores.
export const calculateScore = (pointA: LatLng, pointB: LatLng, countryCode: string | null): number => {
    const toRadians = (degrees: number) => (degrees * Math.PI) / 180;

    const lat1 = toRadians(pointA.lat);
    const lng1 = toRadians(pointA.lng);
    const lat2 = toRadians(pointB.lat);
    const lng2 = toRadians(pointB.lng);

    const dLat = lat2 - lat1;
    const dLng = lng2 - lng1;

    // haversine (?) forumla.
    const a =
        Math.sin(dLat / 2) * Math.sin(dLat / 2) +
        Math.cos(lat1) * Math.cos(lat2) *
        Math.sin(dLng / 2) * Math.sin(dLng / 2);
    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
    const maxDistance = Math.PI; // Half the circumference of the sphere (in radians)

    // 0-1 distance on the earth
    const normalizedDistance = c / maxDistance;

    // factor encouraging better scores (?)
    const p = 4;

    return 100 * Math.exp(-normalizedDistance);
};
