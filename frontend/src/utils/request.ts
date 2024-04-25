import { sleep } from "./debug";
import { getUrl } from "./url";

const api = "http://localhost:5000"; // or whatever the backend URL is
const suffix = "/v3/api"; // or whatever the backend API  suffix is

/**
 * Simple Typescript function to make GET requests to the backend
 * From: https://github.dev/blekhmanlab/compendium_website
 */
export const requestSimple = async <Type>(url: string, type: "json" | "text" = "json") => {
  const options: RequestInit = { redirect: "follow" };
  const response = await fetch(url, options);
  if (!response.ok) throw Error("Response not OK");
  if (type === "text") return (await response.text()) as Type;
  else return (await response.json()) as Type;
};

/**
 * More complex Typescript function to make GET and POST requests to the backend
 * From: https://github.com/monarch-initiative/monarch-app/
 */
type Param = string | number | boolean | undefined | null;
export type Params = { [key: string]: Param | Param[] };

/** session response cache */
const cache = new Map<string, Response>();

/** generic fetch request wrapper */
export const request = async <Response>(
  /** request url */
  path = "",
  /**
   * key/value object for url parameters.
   * use primitive for single, array for multiple/duplicate.
   *    { ids: "1,2,3" } -> ?ids=1,2,3
   * or
   *    { id: [1,2,3] } -> ?id=1&id=2&id=3
   */
  params: Params = {},
  /** fetch options */
  options: RequestInit = {},
  /** parse response mode */
  parse: "text" | "json" = "json",
  /** whether open request url in new tab */
  newTab = false
): Promise<Response> => {
  /** get string of url parameters/options */
  const paramsObject = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    const values = [value].flat();
    for (const value of values) if (["string", "number", "boolean"].includes(typeof value)) paramsObject.append(key, String(value));
  }

  /**artificial delay */
  await sleep();

  /** sort params for consistency */
  paramsObject.sort();
  console.debug("paramsObject: ", paramsObject.toString());

  /** assemble url to query */
  // const url = api + path;
  const paramsString = "?" + paramsObject.toString();
  const url = api + path + "/" + paramsString;

  if (newTab) window.open(url);

  /** make request object */
  const request = new Request(url, options);

  /** unique request id */
  const id = JSON.stringify(request, ["url", "method", "headers"]);

  /** first check if request is cached */
  let response = cache.get(path);

  /** logging info */
  const cached = response ? "cached" : "new";
  const endpoint = getUrl(path, "pathname").replace(suffix, "");

  if (import.meta.env.MODE !== "test")
    console.debug(`📞 Request (${cached}) ${path}`, {
      url,
      params,
      options,
      request,
    });

  /** if request not cached, make new request */
  if (!response) {
    response = await fetch(url, options);
    console.debug("response: ", response);
  }

  /** capture error for throwing later */
  let error = "";

  /** check response code */
  if (!response.ok) {
    // error = `Response not OK`;
    switch (response.status) {
      case 401:
        error = `Unauthorized`;
        break;
      case 403:
        error = `Forbidden`;
        break;
      case 404:
        return parse === "text" ? ("Event not found" as unknown as Response) : ({ error: "Event not found" } as unknown as Response);
      case 500:
        error = `Internal Server Error`;
        break;
      case 502:
        error = `Bad Gateway`;
        break;
      case 503:
        error = `Service Unavailable`;
        break;
      case 504:
        error = `Gateway Timeout`;
        break;
      default:
        error = `Response not OK`;
        break;
    }
  }

  /** parse response */
  let parsed: Response | undefined;
  try {
    parsed = parse === "text" ? await response.clone().text() : await response.clone().json();
  } catch (e) {
    error = `Couldn't parse response as ${parse}`;
  }

  if (import.meta.env.MODE !== "test")
    console.debug(`📣 Response (${cached}) ${path}`, {
      url,
      params,
      options,
      parsed,
      response,
    });

  /** throw error after details have been logged */
  if (error || parsed === undefined) throw Error(error);

  /** add response to cache */
  if (request.method === "GET") cache.set(path, response);

  return parsed;
};
