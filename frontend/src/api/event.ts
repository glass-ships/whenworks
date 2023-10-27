import { Event } from "@/api/model";
import { request } from "@/utils/request";
// import { host } from "@/api/index";

export const getEvent = async (eventID: string) => {
  const response = await request(eventID);
  return response;
};

export const createEvent = async (event: Event) => {
  const options: RequestInit = {
    method: "POST",
    headers: {},
    body: JSON.stringify(event),
    // mode: "no-cors",
  };

  const response = await request("new", {}, options);
  return response;
};
