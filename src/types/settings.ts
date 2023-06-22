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

export class Settings {
    api_key: string;
    google_api_key: string;
    min_waiting_time: number;
    max_waiting_time: number;
    max_transfers: number;
    max_walking_time: number;
    walking_speed: number;

    // In the advanced section
    search_interval: number;

    toString(): string {
        return (
            "{ " +
            `api_key: ${this.api_key}, ` +
            `google_api_key: ${this.google_api_key}, ` +
            `min_waiting_time: ${this.min_waiting_time}; ` +
            `max_waiting_time: ${this.max_waiting_time}; ` +
            `max_transfers: ${this.max_transfers}; ` +
            `max_walking_time: ${this.max_walking_time}; ` +
            `walking_speed: ${this.walking_speed} ` +
            `search_interval: ${this.search_interval} ` +
            "}"
        );
    }

    constructor(
        api_key: string,
        google_api_key: string,
        min_waiting_time: number,
        max_waiting_time: number,
        max_transfers: number,
        max_walking_time: number,
        walking_speed: number,
        search_interval: number,
    ) {
        return {
            api_key: api_key,
            google_api_key: google_api_key,
            min_waiting_time: min_waiting_time,
            max_waiting_time: max_waiting_time,
            max_transfers: max_transfers,
            max_walking_time: max_walking_time,
            walking_speed: walking_speed,
            search_interval: search_interval,
        };
    }
}
