// Function that returns the current date in the format YYYY-MM-DD
export const getCurrentDate = () => {
  const today = new Date();
  const year = today.getFullYear();
  const month = today.getMonth() + 1;
  const day = today.getDate();

  return `${year}-${month}-${day}`;
};


// Function to get current date as timestamp
export const getCurrentTimestamp = () => {
  return Date.now();
};


// Convert timestamp to date in the format YYYY-MM-DD
export const convertTimestampToDate = (timestamp: number) => {
  const date = new Date(timestamp);
  const year = date.getFullYear();
  const month = date.getMonth() + 1;
  const day = date.getDate();

  return `${year}-${month}-${day}`;
};
