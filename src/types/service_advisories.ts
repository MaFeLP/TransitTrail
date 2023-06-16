/**
 * Structures for the [service_advisories endpoint](crate::endpoints::service_advisories)
 */

/**
 * A service advisory containing data about the advisory
 */
export interface ServiceAdvisory {
    /**
     * A unique key to identify the advisory
     */
    key: number;

    /**
     * An indicator of how urgent the advisory is
     */
    priority: Priority;

    /**
     * A title ascribed to the advisory.
     */
    title: string;

    /**
     * The content of the advisory.
     */
    body: string;

    /**
     * Service advisories belong to a category
     */
    category: Category;

    /**
     * Timestamp of when the advisory was last updated.
     */
    updated_at: Date;
}

/**
 * A numerical indicator of how urgent the advisory is. The lower the number, the more urgent it is
 */
export enum Priority {
    /**
     * Priority of this advisory is very high
     */
    VeryHigh = 1,

    /**
     * Priority of this advisory is high
     */
    High = 2,

    /**
     * Priority of this advisory is medium
     */
    Medium = 3,

    /**
     * Priority of this advisory is low
     */
    Low = 4,

    /**
     * Priority of this advisory is very low
     */
    VeryLow = 5,
}

/**
 * Service advisories belong to a category
 */
export enum Category {
    /**
     * Only transit vehicles are effected
     */
    Transit = "transit",

    /**
     * Only Handi-Transit vehicles are effected
     */
    HandiTransit = "handi-transit",

    /**
     * Both Transit and HandiTransit are effected
     */
    All = "all",
}
