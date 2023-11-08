// import { useState } from "react";
import { getEvent } from "@/api/event";
import Header from "@/sections/Header";
import EventForm from "@/sections/EventForm";
import "./App.css";

function App() {
  // const [count, setCount] = useState(0);

  // Test endpoints
  // const event = getEvent('Wu0rwE'); // bad event
  // getEvent("XkMSUY"); // good event

  return (
    <>
      <Header />
      <h2>Find a time that works for everyone</h2>
      <main>
        <div className="card">
          <h3>Create a New Event</h3>
          <EventForm />
          {/* <p>
            <button
            onClick={() => {
              setCount(count + 1);
              console.log("count", count);
            }}
            >
            Create event
            </button>
          </p> */}
        </div>
      </main>
    </>
  );
}

export default App;
