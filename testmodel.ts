// 16 bytes, base64url encoded, no padding char
export type Id = string;

export interface Event {
    /** The name of the event */
    name: string,
    /** A description of the event */
    desc?: string,
    /** The dates of the event */
    dates?: DateRange[],
    /** The users of the event */
    users?: User[],
};

export interface User {
    /** The name of the user */
    name: string,
    /** A comment from the user */
    comment?: string,
    /** The availability dates of the user */
    dates?: DateRange[],
};

export interface DateType {
    /** The start date of the availability */
    from: number,
    /** The end date of the availability */
    to: number,
    /** Whether the user prefers this date */
    preferred: boolean,
};

