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
