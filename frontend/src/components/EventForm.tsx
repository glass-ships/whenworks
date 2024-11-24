// First pass at an event creation form
// Basic implementation, no library usage yet
import { FormEvent, useState } from "react";
import classes from "./EventForm.module.css";
import Textbox from "@/components/Textbox";
import { Event, DateType, User } from "@/api/model";
import { createEvent } from "@/api/event";
import { Link } from "react-router";
// import Button from "@/components/Button";

export default function EventForm() {
  const [name, setName] = useState("");
  const [desc, setDesc] = useState("");
  const [dates, setDates] = useState("");

  const onSubmit = (event: any) => {
    event.preventDefault();
    const dt = new Date().valueOf();
    const data: Event = {
      name: name,
      desc: desc,
      creation_date: dt,
      dates: [],
      // users: {},
      users: [],
    };
    console.log("Input data: ", data);
    const eventResponse = createEvent(data);
    console.log("Response: ", eventResponse);
  };

  return (
    <div>
      <h1>We're still working on it.</h1>
      <h2>
        Check the <Link to={"./testbed"}>testbed</Link> to play!
      </h2>
    </div>
  );
}
