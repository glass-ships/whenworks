/** wait ms */
export const sleep = async (ms = 0): Promise<void> =>
  new Promise((resolve) => globalThis.setTimeout(resolve, ms));

/** pretty log collection of things as object 
 * from monarch-app/frontend/src/api/index.ts
*/
export const groupLog = (label: string, object: { [key: string]: unknown }) => {
    console.groupCollapsed(label);
    for (const [key, value] of Object.entries(object)) {
      console.info("%c" + key, "font-weight: bold");
      console.info(value);
    }
    console.groupEnd();
  };