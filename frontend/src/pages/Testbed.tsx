import React, {
  // FormEvent,
  useState,
} from "react";
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
// import AppSection from "@/components/AppSection";

export default function Testbed() {
  const [name, setName] = useState("");
  const [desc, setDesc] = useState("");
  const [eventId, setEventId] = useState("");
  const [event, setEvent] = useState<Event>();
  const [eventResponse, setEventResponse] = useState<EventResponse>();
  // const [dates, setDates] = useState<DateType[]>([]);

  const onSubmit = async (event: React.FormEvent) => {
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

  const getEventById = async (event: React.FormEvent) => {
    // event.preventDefault();
    setEvent(await getEvent(eventId));
  };

  return (
    <main>
      <h1>WhenWorks Testbed</h1>

      <div style={{ padding: "2rem" }}>
        <h2> Create a New Event </h2>
        <form className={classes.form} onSubmit={onSubmit}>
          <Textbox value={name} onChange={setName} placeholder="Event Name" />
          <Textbox type="textarea" value={desc} onChange={setDesc} placeholder="Event Description" />
          {/* DATE SELECTION GOES HERE */}
          <button className={classes.button} type="submit" onSubmit={(e) => onSubmit(e)}>
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

      <div style={{ padding: "2rem" }}>
        <h2>Get Event</h2>
        <form className={classes.form}>
          <Textbox value={eventId} onChange={setEventId} placeholder="Event ID" />
          <button className={classes.button} type="submit" onSubmit={getEventById}>
            Get event
          </button>
        </form>
        {event && (
          <div>
            <h3>Event: {event.name}</h3>
            <p>Creation Date: {event.creation_date}</p>
            <p>Description: {event.desc}</p>
            <p>Dates: {event.dates?.map((date) => date.toString())}</p>
            <p>Users: {event.users?.map((user) => user.username)}</p>
          </div>
        )}
      </div>
    </main>
  );
}
