export type UserUsername = string;



export interface Event {
    /** The name of the event */
    name: string,
    /** A description of the event */
    desc?: string,
    /** The date the event was created */
    creation_date: number,
    /** The dates of the event */
    dates?: DateType[],
    /** The users of the event */
    users?: {[index: UserUsername]: User },
};

export interface User {
    /** The name of the user */
    username: string,
    /** A comment from the user */
    comment?: string,
    /** The availability dates of the user */
    avail_dates?: DateType[],
};

export interface DateType {
    /** The start date of the availability */
    from: number,
    /** The end date of the availability */
    to: number,
    /** Whether the user prefers this date */
    preferred: boolean,
};

