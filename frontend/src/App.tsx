import { useState } from "react";
import reactLogo from "./assets/react.svg";
// import viteLogo from "/vite.svg";
import axios from 'axios';
import Header from "./sections/Header";
import "./App.css";

//  from https://github.dev/blekhmanlab/compendium_website
// export const request = async <Type>(
//   url: string,
//   type: "json" | "text" = "json",
// ) => {
//   const options: RequestInit = { redirect: "follow" };
//   const response = await fetch(url, options);
//   if (!response.ok) throw Error("Response not OK");
//   if (type === "text") return (await response.text()) as Type;
//   else return (await response.json()) as Type;
// };

function App() {
  const [count, setCount] = useState(0);

  axios.get('http://localhost:8080').then((response) => {
    console.log(response);
  }
  );

  return (
    <>
      <Header />
      <div>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>count is {count}</button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">Click on the Vite and React logos to learn more</p>
    </>
  );
} 

export default App;
