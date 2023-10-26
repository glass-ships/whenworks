import { request } from "@/utils/request";

export const getEvent = async (eventID: string) => {
  const response = await request(eventID);
  return response;
};

export const createEvent = async (event: any) => {
  const options = {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Accept": "application/json"
    },
    body: JSON.stringify(event)
  };
  const response = await request(event, {}, options);
  return response;
};
