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
    walking_distance: number;
    waiting_time: number;
    walking_speed: number;

    toString(): string {
        return (
            "{" +
            `api_key: ${this.api_key}, ` +
            `walking_distance: ${this.walking_distance}, ` +
            `waiting_time: ${this.waiting_time}, ` +
            `walking_speed: ${this.walking_speed} ` +
            "}"
        );
    }
}
