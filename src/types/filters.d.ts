/**
 * Filters for various API endpoints, that may have more than one filter option
 */

import { StreetLeg, StreetType } from "./common";
import { Category, Priority } from "./service_advisories";

/**
 * Filter service advisories
 */
export type ServiceAdvisoryFilter =
    | {
          /**
           * Only return service advisories of this priority or higher. (default: [Priority.VeryLow])
           */
          Priority: Priority;
      }
    | {
          /**
           * Only return service advisories of this category (default: [Category.All])
           */
          Category: Category;
      }
    | {
          /**
           * Only returns advisories created or updated in the last N days.
           */
          MaxAge: number;
      }
    | {
          /**
           * Only show the top N service advisories -- no more than the given limit.
           */
          Limit: number;
      };

/**
 * Specify filters for the trip planning
 */
export type TripPlanFilter =
    | {
          /**
           * The date for which to get navigo results. Defaults to today, if not included as a filter
           */
          Date: Date;
      }
    | {
          /**
           * The time of the trip. Defaults to now, if not included as a filter.
           *
           * What the time means can be customized with a [Mode]
           */
          Time: Time;
      }
    | {
          /**
           * The mode with which the trip should be planned
           *
           * What the time applies to: If the time specifies where to be when, or when to leave
           */
          Mode: Mode;
      }
    | {
          /**
           * Walking speed in km/h.
           */
          WalkSpeed: number;
      }
    | {
          /**
           * The maximum number of minutes to spend walking.
           */
          MaxWalkTime: number;
      }
    | {
          /**
           * The minimum number of minutes to spend waiting for a transfer.
           */
          MinTransferWait: number;
      }
    | {
          /**
           * The maximum number of minutes to spend waiting for a transfer.
           */
          MaxTransferWait: number;
      }
    | {
          /**
           * The maximum number of total transfers.
           */
          MaxTransfers: number;
      };

/**
 * What the time applies to: If the time specifies where to be when, or when to leave
 */
export enum Mode {
    /**
     * Depart before the given time.
     */
    DepartBefore = "depart-before",

    /**
     * Depart after the given time.
     */
    DepartAfter = "depart-after",

    /**
     * Arrive before the given time.
     */
    ArriveBefore = "arrive-before",

    /**
     * Arrive after the given time.
     */
    ArriveAfter = "arrive-after",
}

/**
 * A filter when searching for streets
 */
export type Street =
    | {
          /**
           * Filter for the name of the street
           */
          Name: string;
      }
    | {
          /**
           * Filter for the type of the street
           */
          Type: StreetType;
      }
    | {
          /**
           * Filter for the leg of the street
           */
          Leg: StreetLeg;
      };

/**
 * A filter when getting the schedule for a stop
 */
export type Stop =
    | {
          /**
           * Only return results for the specified route
           *
           * Defaults to all routes
           */
          Routes: number[];
      }
    | {
          /**
           * Only return results after this time
           *
           * Defaults to now
           */
          Start: [number, number];
      }
    | {
          /**
           * Only return results before this time
           *
           * Defaults to two hours from now
           */
          End: [number, number];
      }
    | {
          /**
           * Limit the results per returned route
           */
          MaxResultsPerRoute: number;
      };
