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

/* eslint-disable @typescript-eslint/no-explicit-any */

import { error as errorLog, info as infoLog } from "tauri-plugin-log-api";

export function info(message: string, ...args: any[]) {
    infoLog(message)
        .then(() => console.log(message, args))
        .catch((err) => console.error("Could not call log function!", err));
}

export function error(message: string, ...args: any[]) {
    errorLog(message)
        .then(() => console.error(message, args))
        .catch((err) => console.error("Could not call log function!", err));
}
