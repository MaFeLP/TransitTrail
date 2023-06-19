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
