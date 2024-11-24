import { FormEvent, useState } from "react";
import classes from "./Testbed.module.css";
import Textbox from "@/components/Textbox";
import {
  Event,
  EventResponse,
  // DateType,
  // User
} from "@/api/model";
import { createEvent, getEvent } from "@/api/event";
// import Button from "@/components/Button";

const EventForm = () => {
  const [name, setName] = useState("");
  const [desc, setDesc] = useState("");
  const [eventResponse, setEventResponse] = useState<EventResponse>();
  const [eventId, setEventId] = useState("");
  // const [dates, setDates] = useState<DateType[]>([]);

  const onSubmit = async (event: any) => {
    event.preventDefault();
    const dt = Math.floor(new Date().valueOf() / 1000);
    const data: Event = {
      name: name,
      desc: desc,
      creation_date: dt,
      dates: [],
      // users: {},
      users: [],
    };
    setEventResponse(await createEvent(data));
  };

  const getEvent = (event: any) => {
    event.preventDefault();
    const eventResponse = getEvent(eventId);
    console.log("Response: ", eventResponse);
  };

  return (
    <div>
      <h1>WhenWorks Testbed</h1>

      <h2> Create Event </h2>
      <div>
        <form className={classes.form} onSubmit={onSubmit}>
          <Textbox value={name} onChange={setName} placeholder="Event Name" />
          <Textbox type="textarea" value={desc} onChange={setDesc} placeholder="Event Description" />
          {/* <Textbox value={dates} onChange={setDates} placeholder="Event Dates" /> */}
          <button className={classes.button} type="submit" onSubmit={onSubmit}>
            Create event
          </button>
        </form>
        {eventResponse && (
          <div>
            <h3>Event created!</h3>
            <p>Key: {eventResponse.key}</p>
            <p>UID: {eventResponse.uid}</p>
          </div>
        )}
      </div>

      <h2>Get Event</h2>
      <form className={classes.form}>
        <Textbox value={eventId} onChange={setEventId} placeholder="Event ID" />
        <button className={classes.button} type="submit" onSubmit={() => getEvent(eventId)}>
          Get event
        </button>
      </form>
    </div>
  );
};

export default EventForm;
