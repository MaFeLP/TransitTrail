/**
 * Data structures for the stops endpoint
 */

import type { GeoLocation, Street } from "./common";
import type { Coverage, Variant } from "./routes";

/**
 * A stop
 */
export interface Stop {
    /**
     * A unique identifier for this stop.
     */
    key: number;

    /**
     * The stop name
     */
    name: string;

    /**
     * The stop number
     */
    number: number;

    /**
     * When a location was specified, these are the distances it takes
     * to get to the stop.
     */
    distances?: Distances;

    /**
     * Specifies which direction buses which service the stop are heading.
     */
    direction: Direction;

    /**
     * Specifies which side of the intersection the stop lies on.
     */
    side: Side;

    /**
     * The street on which the stop is located
     */
    street: Street;

    /**
     * The street that intersects the main [Street]
     */
    "cross-street": Street;

    /**
     * A geographical point describing where the stop is located.
     */
    centre: GeoLocation;

    /**
     * The internal name use by the API
     */
    "internal-name"?: string;

    /**
     * The how many-th stop on the street this stop is
     */
    "sequence-on-street"?: number;

    /**
     * What icon style to use
     */
    "icon-style"?: string;
}

/**
 * A Stops that only contains the minimum required.
 */
export interface PartialStop {
    /**
     * The stop number
     */
    id: number;

    /**
     * Where the stop is located
     */
    position: GeoLocation;

    /**
     * What style/image the icon should have
     */
    iconStyle: PartialStopIconStyle;
}

/**
 * What style a PartialStop has
 */
export enum PartialStopIconStyle {
    /**
     * It is a normal stop, with the Blue Winnipeg Transit Logo
     */
    Blue = "Blue",

    /**
     * The Partial Stop is a RapidTransit stop, with the rt-logo
     */
    Rt = "Rt",
}

/**
 * Distances in meters to the stop
 */
export interface Distances {
    /**
     * The direct distance to the stop
     */
    direct: number;

    /**
     * The distance it takes to walk there
     */
    walking: number;
}

/**
 * Specifies which direction buses which service the stop are heading.
 */
export enum Direction {
    /**
     * The bus is going North
     */
    Northbound = "Northbound",

    /**
     * The bus is going East
     */
    Eastbound = "Eastbound",

    /**
     * The bus is going South
     */
    Southbound = "Southbound",

    /**
     * The bus is going West
     */
    Westbound = "Westbound",
}

/**
 * Specifies which side of the intersection the stop lies on.
 */
export enum Side {
    /**
     * The stop is directly on the opposite side
     *
     * **Example**: `10168`
     */
    DirectOpposite = "Direct Opposite",

    /**
     * The stop is on the far side
     *
     * **Example**: `10095`
     */
    Farside = "Farside",

    /**
     * The stop is on the far- and opposite side of the street
     *
     * **Example**: `10081`
     */
    FarsideOpposite = "Farside Opposite",

    /**
     * The stop is on the nearside of the street
     *
     * **Example**: `10076`
     */
    Nearside = "Nearside",

    /**
     * The stop is on the near- and opposite side of the street
     *
     * **Example**: `10077`
     */
    NearsideOpposite = "Nearside Opposite",

    /**
     * No side of the street available for this stop
     *
     * **Example**: `10087`
     */
    NA = "N/A",
}

/**
 * information about any stop features
 *
 * This includes: Benches, (Un-)heated shelters, etc.
 */
export interface Feature {
    /**
     * The name of the stop feature
     */
    name: string;

    /**
     *  The number of occurrences of the feature at this stop
     */
    count: number;
}

/**
 * A schedule of what buses are leaving from this stop
 */
export interface Schedule {
    /**
     * The stop which the schedule information is for. See the [Stop] for more details.
     */
    stop: Stop;

    /**
     * A route schedule is returned for each route which services the stop.
     */
    "route-schedules": RouteSchedule[];
}

/**
 * A route schedule of a route and where it is going.
 */
export interface RouteSchedule {
    /**
     * Basic route information.
     */
    route: FoxxRoute;

    /**
     * Contains information about when a bus on the given route will pass by the stop.
     */
    "scheduled-stops": ScheduledStop[];
}

/**
 * Contains information about when a bus will pass by the stop.
 */
export interface ScheduledStop {
    /**
     * A unique identifier for this scheduled-stop.
     */
    key: string;

    /**
     * Boolean field describing whether or not this scheduled stop has been cancelled.
     */
    cancelled: boolean;

    /**
     * Times of when the bus is schedules/estimated to arrive/departure
     */
    times: ScheduledTimes;

    /**
     * The variant of the route which the passing bus belongs to. See the Variant for more details.
     */
    variant: Variant;

    /**
     * Information about the passing bus. Will typically be present in today's schedule results and omitted for past and future dates.
     */
    bus?: Bus;
}

/**
 * Information about the arrival and departure times
 */
export interface ScheduledTimes {
    /**
     * Times of when the bus is scheduled and estimated to arrive
     */
    arrival: Time;

    /**
     * Times of when the bus is scheduled and estimated to depart
     */
    departure: Time;
}

/**
 * Holds scheduled and estimated times for departure or arrival
 */
export interface Time {
    /**
     * When the bus is scheduled
     */
    scheduled: Date;

    /**
     * When the bus is estimated
     */
    estimated: Date;
}

/**
 * Information about the passing bus. Will typically be present in today's schedule results
 *
 * and omitted for past and future dates.
 */
export interface Bus {
    /**
     * A unique identifier for the bus.
     */
    key: number;

    /**
     * Whether or not the bus has a bike rack
     */
    bike_rack: boolean;

    /**
     * Whether or not the bus has wifi
     */
    wifi: boolean;
}

/**
 * A busses route.
 *
 * Author: [Foxx](mailto:f.pinkerton@sjsad.ca)
 */
export interface FoxxRoute {
    /**
     * The bus key
     */
    key: number | "BLUE";

    /**
     * The bus number
     */
    number: number | "BLUE";

    /**
     * The bus name
     */
    name?: string;

    /**
     * The customer's the bus services
     */
    "customer-type": CustomerType;

    /**
     * The bus coverage
     */
    coverage: Coverage;

    /**
     * The Badge Label
     */
    "badge-label": number | "BLUE";

    /**
     * The Badge Style
     */
    "badge-style": BadgeStyle;

    /**
     * The bus variants
     */
    variants?: Variant[];
}

/**
 * The customer's the bus services
 */
export enum CustomerType {
    /**
     * The bus is a regular bus
     */
    Regular = "regular",

    /**
     * The bus is a HandiTransit bus
     */
    HandiTransit = "HandiTransit",

    /**
     * The bus is a DART bus
     */
    DART = "DART",

    /**
     * The bus is a School Charter bus
     */
    School = "School",

    /**
     * The bus is a Community bus
     */
    Community = "Community",

    /**
     * The bus is a Express bus
     */
    Express = "Express",

    /**
     * The bus is a Rapid Transit bus
     */
    RapidTransit = "RapidTransit",

    /**
     * The bus is a Unknown bus
     */
    Unknown = "Unknown",
}

/**
 * Styling for the badge
 */
export interface BadgeStyle {
    /**
     * Classes for the badge
     */
    "class-names": RenameMe;

    /**
     * The background color of the badge
     */
    "background-color": string;

    /**
     * The border color of the badge
     */
    "border-color": string;

    /**
     * The color of the badge
     */
    color: string;
}

/**
 * TODO: Rename me
 */
export interface RenameMe {
    /**
     * The class name
     */
    "class-name": FoxxClassNames[];
}

/**
 * class names
 */
export enum FoxxClassNames {
    /**
     * Lable for the badge
     */
    BadgeLabel = "badge-label",

    /**
     * Express bus
     */
    Express = "express",

    /**
     * Regular bus
     */
    Regular = "regular",

    /**
     * Rapid Transit bus
     */
    RapidTransit = "rapid-transit",

    /**
     * A feeder bus for express and rapid transit busses
     */
    Feeder = "feeder",

    /**
     * A feeder bus for express and rapid transit busses, which is used at peak times
     */
    PeakFeeder = "peak-feeder",
}
