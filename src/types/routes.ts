/**
 * A route is a named and numbered pattern of service that covers a certain geographic area with a
 * consistent method of service delivery.
 */

/**
 * Represents a route
 */
export interface Route {
    /**
     * The unique key of the route, most of the time the same as the number
     */
    key: number;

    /**
     * The line number of the route
     */
    number: number | "BLUE";

    /**
     * The name of the route, e.g. where it's going
     */
    name: string;

    /**
     * Who is buying the route
     */
    "customer-type": Customer;

    /**
     * If the route skips specific stops on the way
     */
    coverage: Coverage;

    /**
     * What is on the badge of the route
     */
    "badge-label": number | "BLUE";

    /**
     * How this route's badge should be styled. For more info, see badges
     */
    "badge-style": BadgeStyle;

    /**
     * Variants of the current route, e.g. if the route splits up, where it's destination is.
     *
     * Is always set on the routes endpoint, but not set in the stop's endpoint
     */
    variants?: Variant[];
}

/**
 * The type of service provided by this route.
 */
export enum Customer {
    /**
     * Regular Service at this route
     */
    Regular = "regular",

    /**
     * Industrial Service
     */
    Industrial = "industrial",

    /**
     * Service for specific schools
     */
    School = "school",

    /**
     * Chartered Buses
     */
    Charter = "charter",

    /**
     * Work buses
     */
    Work = "work",
}

/**
 * Categorization of how fully a route services stops along it's segments.
 */
export enum Coverage {
    /**
     * services all stops
     */
    Regular = "regular",

    /**
     * services select stops in express segments
     */
    Express = "express",

    /**
     * services no stops in express segments
     */
    SuperExpress = "super express",

    /**
     * Special Type of coverage for BLUE routes
     */
    RapidTransit = "rapid transit",

    /**
     * A feeder bus for express and rapid transit buses
     */
    Feeder = "feeder",

    /**
     * A feeder bus for express and rapid transit buses, which is used at peak times
     */
    PeakFeeder = "peak feeder",
}

/**
 * A variant is a variation of a route, distinguished by its intermediate destination points.
 */
export interface Variant {
    /**
     * A unique identifier for this variant.
     */
    key: string;

    /**
     * The variant name.
     */
    name?: string;
}

/**
 * How the route should be styler
 */
export interface BadgeStyle {
    /**
     * Additional classes to apply to nodes for styling
     */
    "class-names": ClassNames;

    /**
     * The background colour of the badge
     */
    "background-color": string;
    /**
     * The colour of the border of the badge
     */
    "border-color": string;
    /**
     * The colour of the line
     */
    color: string;
}

/**
 * Additional class names to apply to nodes for styling
 */
export interface ClassNames {
    /**
     * Additional class names that should be applied
     */
    "class-name": string[];
}
