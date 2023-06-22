// TransitTrail - Navigate Winnipeg Transit with a different style
// Copyright (C) - 2023 Foxx Azalea Pinkerton, Max Fehlinger
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.

/**
 * Represents a geographic location on Earth, defined by latitude and longitude coordinates.
 */
export interface GeoLocation {
    /**
     * The latutude of the point
     */
    latitude: number;

    /**
     * The longitude of the point
     */
    longitude: number;
}

/**
 * Represents the type of a location.
 */
export enum LocationType {
    /**
     * The location is an address
     */
    Address = "address",

    /**
     * The location is a monument
     */
    Monument = "monument",

    /**
     * The location is an intersection
     */
    Intersection = "intersection",

    /**
     * The location is a geographical point
     */
    Point = "point",

    /**
     * The location is a bus stop
     */
    Stop = "stop",
}

/**
 * Represents a location with its associated type.
 */
export interface Location {
    /**
     * What type the location has
     */
    type: LocationType;

    /**
     * The unique key of the location
     */
    key: number | string;

    /**
     * What street the address is located on
     *
     * Used in address, intersection
     */
    street?: Street;

    /**
     * The house number/street number of the address
     *
     * Used in address
     */
    "street-number"?: number;

    /**
     * The geographic centre of the address
     *
     * Used in type address, intersection, point
     */
    centre?: GeoLocation;

    /**
     * What the point of interest is called
     *
     * Used in monument
     */
    name?: string;

    /**
     * Which categories the point of interest/monument has
     *
     * Used in monument
     */
    categories?: string[];

    /**
     * The address of the monument
     *
     * Used in monument
     */
    address?: Address;

    /**
     * The street crossing the main street
     *
     * Used in intersection
     */
    "cross-street"?: Street;

    /**
     * If type is set to <code>point</code>, this represents the geographical coordinates of the location
     */
    point?: GeoLocation;
}

export interface PartialLocation {
    /**
     * The address of a Location
     */
    Address?: string;

    /**
     * The location is a significant point of interest
     */
    Monument?: string;

    /**
     * The location is at an intersection of two streets
     */
    Intersection?: string;

    /**
     * A geographic point, representing latitude and longitude
     */
    Point?: [number, number];

    /**
     * A bus stop
     */
    Stop?: number;
}

export function toPartialLocation(location: Location, transit_api_format = true): PartialLocation {
    if (!transit_api_format) {
        if (location.type === LocationType.Monument) return { Monument: location.name };

        return { Point: [location.centre.latitude, location.centre.longitude] };
    }

    switch (location.type) {
        case LocationType.Address:
            return { Address: location.key.toString() };
        case LocationType.Monument:
            return { Monument: location.key.toString() };
        case LocationType.Intersection:
            return { Intersection: location.key.toString() };
        case LocationType.Point:
            return { Point: [location.point.latitude, location.point.longitude] };
        case LocationType.Stop:
            // A Stop location's key is always a number
            return { Stop: location.key as number };
    }
}

/**
 * Represents a street with its attributes.
 */
export interface Street {
    /**
     * The unique key of the street
     */
    key: number;

    /**
     * The name of the street. Can be more or less verbose, depending on the usage
     * of the usage parameter
     */
    name: string;

    /**
     * Optionally a street type may be specified, e.g. Road, Street, etc.
     *
     * This will be changed to a string a future version
     */
    type?: string;

    /**
     * If this street is split into more than one part, a street leg is given
     */
    leg?: StreetLeg;
}

/**
 * Represents the part of a street if it is split into multiple parts.
 */
export enum StreetLeg {
    North = "North",
    East = "East",
    South = "South",
    West = "West",
}

/**
 * Represents a residential address.
 */
export interface Address {
    /**
     * The unique key of the address
     */
    key: number;

    /**
     * What street the address is located on
     */
    street: Street;

    /**
     * The house number/street number of the address
     */
    "street-number": number;

    /**
     * The geographic centre of the address
     */
    centre: GeoLocation;
}

/**
 * Represents a significant point of interest.
 */
export interface Monument {
    /**
     * The unique key of the point of interest
     */
    key: number;

    /**
     * What the point of interest is called
     */
    name: string;

    /**
     * Which categories the point of interest has
     */
    categories: string[];

    /**
     * The address of the point of interest
     */
    address: Address;
}

/**
 * Represents an intersection of two streets.
 */
export interface Intersection {
    /**
     * The unique key of the intersection.
     * Composed of the unique keys of the two streets
     */
    key: string;

    /**
     * The main street of the crossing streets
     */
    street: Street;

    /**
     * The street crossing the main street
     */
    "cross-street": Street;

    /**
     * The geographic centre of the intersection
     */
    centre: GeoLocation;
}
