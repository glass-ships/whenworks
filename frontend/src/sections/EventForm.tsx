// First pass at an event creation form
// Basic implementation, no library usage yet
import { FormEvent, useState } from "react";
import classes from "./EventForm.module.css";
import Textbox from "@/components/Textbox";
import { Event, DateType, User } from "@/api/model";
import { createEvent } from "@/api/event";
// import Button from "@/components/Button";

const EventForm = () => {
  const [name, setName] = useState("");
  const [desc, setDesc] = useState("");
  const [dates, setDates] = useState("");

  const onSubmit = (event: any) => {
    event.preventDefault();
    let dt = new Date().valueOf();
    const data: Event = {
      name: name,
      desc: desc,
      creation_date: dt,
    };
    console.log("Input data: ", data);
    const eventResponse = createEvent(data);
    console.log("Response: ", eventResponse);
  };

  return (
    <form className={classes.form} onSubmit={onSubmit}>
      <Textbox value={name} onChange={setName} placeholder="Event Name" />
      <Textbox type="textarea" value={desc} onChange={setDesc} placeholder="Event Description" />
      {/* <Textbox value={dates} onChange={setDates} placeholder="Event Dates" /> */}
      <button className={classes.button} type="submit" onSubmit={onSubmit}>
        Create event
      </button>
    </form>
  );
};

export default EventForm;
