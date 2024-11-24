// import { useState } from "react";
import { getEvent } from "@/api/event";
import Header from "@/components/Header";
import EventForm from "@/components/EventForm";
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
          <EventForm />
        </div>
      </main>
    </>
  );
}

export default App;
