export class Settings {
    api_key: string;
    min_waiting_time: number;
    max_waiting_time: number;
    max_transfers: number;
    max_walking_time: number;
    walking_speed: number;

    toString(): string {
        return (
            "{" +
            `api_key: ${this.api_key}, ` +
            `min_waiting_time: ${this.min_waiting_time}; ` +
            `max_waiting_time: ${this.max_waiting_time}; ` +
            `max_transfers: ${this.max_transfers}; ` +
            `max_walking_time: ${this.max_walking_time}; ` +
            `walking_speed: ${this.walking_speed} ` +
            "}"
        );
    }

    constructor(
        api_key: string,
        min_waiting_time: number,
        max_waiting_time: number,
        max_transfers: number,
        max_walking_time: number,
        walking_speed: number,
    ) {
        return {
            api_key: api_key,
            min_waiting_time: min_waiting_time,
            max_waiting_time: max_waiting_time,
            max_transfers: max_transfers,
            max_walking_time: max_walking_time,
            walking_speed: walking_speed,
        };
    }
}
