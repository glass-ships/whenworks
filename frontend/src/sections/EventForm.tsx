// First pass at an event creation form
// Basic implementation, no library usage yet
import { FormEvent, useState } from "react";
import classes from "./EventForm.module.css";
import Textbox from "@/components/Textbox";
import { Event } from "@/api/model";
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
      event_description: desc,
      creation_date: dt,
    };
    console.log(data);
  };

  return (
    <form className={classes.form} onSubmit={onSubmit}>
      <Textbox value={name} onChange={setName} placeholder="Event Name" />
      <Textbox value={desc} onChange={setDesc} placeholder="Event Description" />
      <Textbox value={dates} onChange={setDates} placeholder="Event Dates" />
      <button className={classes.button} type="submit" onSubmit={onSubmit}>
        Create event
      </button>
    </form>
  );
};

export default EventForm;
