# WhenWorks backend

## Setup

Run the server with `cargo run`

Edit [main.rs](src/main.rs) to change address and port

## Endpoints

#### get the index:  
- request: `GET /` -> index.html

#### get an event:  
- request: `GET /api/<event_id>` -> `{event_entry}`

#### create new event:  
- request: `POST /api/new` + `{body}` -> `{event_id, edit_key}`  
- body: {name, description, creation_date, date_range}`

#### check validity of a key:  
- request: `POST /api/<event_id>?<edit_key>` -> `{event_entry}`

#### edit an event:  
- request: `POST /api/<event_id>?<edit_key>` + `{body}` -> \_
- body: `{name, description, date_range}`

#### add a user:
- request: `POST /api/<event_id>/usr` + `{body}` -> \_

#### edit a user:
- request: `POST /api/<event_id>/usr?e` + `{body}` -> \_
- body: `{name, comment, avail_dates}`
