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
