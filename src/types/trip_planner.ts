import type { Address, Monument, Intersection, GeoLocation } from "./common";
import type { Route, Variant } from "./routes";
import type { Bus } from "./stops";

/**
 * Each plan describes a different trip or path which can be used to get from the origin to the destination.
 */
export interface Plan {
    /**
     * The how many-th plan this is
     */
    number: number;

    /**
     * Contains start and end times of the plan or segment, including the total duration in
     * minutes. Riding, walking, and waiting totals are also included where appropriate.
     */
    times: Times;

    /**
     * Information about how this plan is structured
     */
    segments: Segment[];
}

/**
 * Time information about the Plan/Segment: when it starts/ends and how much time is spent with what.
 */
export interface Times {
    /**
     * When the ride/walk of the plan/segment starts
     */
    start: Date;

    /**
     * When the ride/walk of the plan/segment end
     */
    end: Date;

    /**
     * How much time is spent on different transport options (walking, riding, waiting, total time)
     */
    durations: Durations;
}

/**
 * Times for how long is spent riding/walking/waiting and total
 */
export interface Durations {
    /**
     * Total time spent. Defaults to 0
     */
    total: number;

    /**
     * Total time spent walking. Defaults to 0
     */
    walking: number;

    /**
     * Total time spent waiting. Defaults to 0
     */
    waiting: number;

    /**
     * Total time spent riding on buses. Defaults to 0
     */
    riding: number;
}

/**
 * The geographic boundaries of the segment/plan
 */
export interface Bounds {
    /**
     * The maximum point
     */
    maximum: GeoLocation;

    /**
     * The minimum point
     */
    minimum: GeoLocation;
}

/**
 * Differentiate between stops at the origin, a stop, or the end of the trip
 */
export type TripStop =
    | {
          /**
           * The segment starts at the origin of the Plan
           */
          Origin: Location;
      }
    | {
          /**
           * The segment starts/ends neither at the start, nor at the end of the Plan.
           *
           * Only includes basic information.
           */
          Stop: Stop;
      }
    | {
          /**
           * The segment ends at the [Plan]'s destination
           */
          Destination: Location;
      };

/**
 * A representation of Location, that is serialized and
 * deserialized as an untagged enum.
 * It represents a position or a point on the map that is significant or by address.
 */
export type Location =
    | {
          /**
           * The address of a Location
           */
          Address: Address;
      }
    | {
          /**
           * The location is a significant point of interest
           */
          Monument: Monument;
      }
    | {
          /**
           * The location is at an intersection of two streets
           */
          Intersection: Intersection;
      }
    | {
          /**
           * A geographic point
           */
          Point: GeoLocation;
      };

/**
 * Basic information about a stop on the Trip.
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
     * A geographical point describing where the stop is located.
     */
    centre: GeoLocation;
}

/**
 * Segments can either be of type [Walk], [Ride] or [Transfer]
 */
export enum SegmentType {
    /**
     * The segment is of type Walk
     */
    Walk = "walk",

    /**
     * The segment is of type Ride
     */
    Ride = "ride",

    /**
     * The segment is of type Transfer
     */
    Transfer = "transfer",
}

export interface Segment {
    /**
     * What type this segment is. Depending on this field, other properties are present.
     */
    type: SegmentType;

    /**
     * Shows the boundaries of the trip
     *
     * Present in Walk, Ride, Transfer
     */
    bounds?: Bounds;

    /**
     * Indicates whether the walk path starts at the origin of the trip, or at a stop.
     * Contains location elements, or point elements which define a geographical point.
     *
     * Present in Walk, Transfer
     */
    from?: TripStop;

    /**
     * Individual times for walking and total. Includes default (0) values for all other fields.
     *
     * Present in Walk
     */
    times?: Times;

    /**
     * Indicates whether the walk path ends at the destination of the trip, or at a stop.
     * Contains location elements, or point elements which define a geographical point.
     *
     * Present in Walk, Transfer
     */
    to?: TripStop;

    /**
     * Information about the bus servicing this segment.
     * Typically present in plans for today but omitted for past and future dates.
     *
     * Present in Ride
     */
    bus?: Bus;

    /**
     * The route this bus takes
     *
     * Present in Ride
     */
    route?: Route;

    /**
     * The variant of the bus that is servicing this route
     *
     * Present in Ride
     */
    variant?: Variant;
}
