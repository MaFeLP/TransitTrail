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
