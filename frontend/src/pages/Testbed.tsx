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
import { createEvent, getEvent, editEvent } from "@/api/event";
// import Button from "@/components/Button";
import AppGrid from "@/components/AppGrid";

export default function Testbed() {
  const [name, setName] = useState("");
  const [desc, setDesc] = useState("");
  const [eventId, setEventId] = useState("");
  const [eventKey, setEventKey] = useState("");
  const [foundEvent, setFoundEvent] = useState<Event>({ name: "", creation_date: 0 });
  const [eventResponse, setEventResponse] = useState<EventResponse>();
  const [editResponse, setEditResponse] = useState<EventResponse>();
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
    event.preventDefault();
    // setFoundEvent(await getEvent(eventId));
    const response = await getEvent(eventId);
    console.debug({ response });

    setFoundEvent(response);
  };

  const updateEvent = async (event: React.FormEvent) => {
    event.preventDefault();
    const dt = Math.floor(new Date().valueOf() / 1000);
    const data: Event = {
      name: name,
      desc: desc,
      creation_date: dt,
      dates: [],
      users: [],
    };
    setEditResponse(await editEvent(eventKey, data));
  }

  return (
    <main>
      <h1>WhenWorks Testbed</h1>

      <AppGrid>
        <div style={{ padding: "2rem" }}>
          <h2> Create a New Event </h2>
          <form className={classes.form} onSubmit={onSubmit}>
            <Textbox value={name} onChange={setName} placeholder="Event Name" />
            <Textbox type="textarea" value={desc} onChange={setDesc} placeholder="Event Description" />
            {/* DATE SELECTION GOES HERE */}
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

        <div style={{ padding: "2rem" }}>
          <h2>Get Event</h2>
          <form className={classes.form} onSubmit={getEventById}>
            <Textbox value={eventId} onChange={setEventId} placeholder="Event ID" />
            <button className={classes.button} type="submit">
              Get event
            </button>
          </form>
          {foundEvent.name && (
            <div>
              <h3>Event found!</h3>
              <p>Event: {foundEvent.name}</p>
              <p>Creation Date: {foundEvent.creation_date}</p>
              <p>Description: {foundEvent.desc}</p>
              <p>Dates: {foundEvent.dates?.map((date) => date.toString())}</p>
              <p>Users: {foundEvent.users?.map((user) => user.username)}</p>
            </div>
          )}
        </div>

        <div style={{ padding: "2rem" }}>
          <h2>Edit Event</h2>
          <form className={classes.form} onSubmit={updateEvent}>
            <Textbox value={eventKey} onChange={setEventKey} placeholder="Event Key" />
            <Textbox value={name} onChange={setName} placeholder="New Event Name" />
            <Textbox type="textarea" value={desc} onChange={setDesc} placeholder="New Event Description" />
            <button className={classes.button} type="submit">
              Edit event
            </button>
          </form>
          {editResponse && (
            <div>
              <h3>Event edited!</h3>
              <p>Key: {editResponse.key}</p>
              <p>UID: {editResponse.uid}</p>
            </div>
          )}
        </div>
      </AppGrid>
    </main>
  );
}
