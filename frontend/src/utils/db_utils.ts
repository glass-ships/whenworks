// import axios from "axios";

// const API_URL = "http://localhost:8080/api";

// export const get = async (path: string) => {
//   try {
//     const response = await axios.get(`${API_URL}${path}`);
//     return response.data;
//   } catch(error) {
//     console.log(error);
//   };

// };

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
