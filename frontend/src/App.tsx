import { useState } from "react";
// import { request } from "@/utils/request";
import { getEvent } from "@/utils/db_utils";
import Header from "@/sections/Header";
import "./App.css";


function App() {
  const [count, setCount] = useState(0);

  // const r = requestSimple('http://localhost:8080/api/Wu0rwE') 
  // const r = request('Wu0rwE');
  // r.then((response) => {
  //   console.log(response);
  // });

  const event = getEvent('Wu0rwE');
  console.log(event);

  return (
    <>
      <div>
        <Header />
        <h2>Find a time that works for everyone</h2>
      </div>
      <div className="card">
        <h3>Create a New Event</h3>
        <div>
          Form goes here
        </div>
        <p>
          <button
            onClick={() => {
              setCount(count + 1);
              console.log("count", count);
            }}
          >
            Create event
          </button>
        </p>
      </div>
    </>
  );
} 

export default App;
