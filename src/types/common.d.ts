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
     * If type is set to <code>address</code>, this represents the address which is the location
     */
    address?: Address;

    /**
     * If type is set to <code>monument</code>, this represents the monument, which is the location
     */
    monument?: Monument;

    /**
     * If type is set to <code>intersection</code>, this represents the intersection, which is the location
     */
    intersection?: Intersection;

    /**
     * If type is set to <code>point</code>, this represents the geographical coordinates of the location
     */
    point?: GeoLocation;
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
    type?: StreetType;

    /**
     * If this street is split into more than one part, a street leg is given
     */
    leg?: StreetLeg;
}

/**
 * Represents the type of a street.
 *
 * @deprecated
 */
export enum StreetType {
    Avenue = "Avenue",
    Boulevard = "Boulevard",
    Crescent = "Crescent",
    Drive = "Drive",
    Loop = "Loop",
    Road = "Road",
    Street = "Street",
    Way = "Way",
    Terminal = "Terminal",
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
