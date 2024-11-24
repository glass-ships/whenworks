import { Event, EventResponse } from "@/api/model";
import { request } from "@/utils/request";
// import { host } from "@/api/index";

export const getEvent = async (eventID: string) => {
  const response = await request<Event>(`/${eventID}`);
  return response;
};

export const createEvent = async (event: Event) => {
  const options: RequestInit = {
    method: "POST",
    headers: {
      "Access-Control-Allow-Origin": "*",
      // "Access-Control-Allow-Methods": "GET, POST, PATCH, PUT, DELETE, OPTIONS",
      // "Access-Control-Request-Methods": "POST",
      "Access-Control-Allow-Headers": "*",
      "Content-Type": "application/json",
      Accept: "application/json",
    },
    body: JSON.stringify(event),
    mode: "cors",
  };

  const response = await request<EventResponse>("/new", {}, options);
  return response;
};
