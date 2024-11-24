## Notes and TODOs

- currently there's no way of changing the user's password.
  - Do we even want that? the passwords aren't exactly meant to be super secure,
    and you have to get a new one for each new event anyway.
- new UIDs can technically point to a previously existing event.
  - This would mean that the old event links would now point to the new event.
  - Dont think this is something that needs to be fixed tho as it's exceedingly unlikely (1 in 2^128).

## env vars

- `MAX_EVENT_AGE`: u64 - max age of an event in seconds (default: 30 days)
- `BOOKKEEPING_INTERVAL`: u64 - interval in seconds between bookkeeping runs (default: 30min)
- `DB_PATH`: string - path to the db file (default: `./db.bin`)

## Definitions

(this is a simplified pseudocode interface, the actual implementation will be more complex)

```rs
type Timestamp = u64;

// 16 bytes, base64url encoded, no padding char
type Id = string;

struct Event {
	name:  string,
	creation_date: Timestamp,
	desc:  ?string,
	dates: [DateRange],
	users: [User],
}

struct DateRange {
	from: Timestamp,
	to:   Timestamp,
	preferred: bool,
}

struct User {
	name:    string,
	comment: ?string,
	dates:   [DateRange],
}
```

## Endpoints

**Request:**

> `GET /api/{Id}` - Get an event by id.

**Response:**

> body: `Event`

**Errors:**
> 404: Event with given id does not exist.
> 400: Id is invalid base64 or is an invalid length.

---

**Request:**

> `POST /api/new` - Create a new event.  
> body: `Event`

**Response:**

> body: `{ "uid": Id, "key": Id }`

- uid: the unique event id to share with others. 
- key: the key to edit the event (should be kept secret).

**Errors:**

> 400: Name too long (max 32 chars).  
> 400: Name is empty.  
> 400: Description too long (max 256 chars).

---

**Request:**

> `POST /api/{Id}/edit` - Edit an event by id.  
> body: `[key: Id, Event]`

**Errors:**

> 400: Id is invalid base64.  
> 400: key is invalid base64.  
> 404: Event with given id does not exist.  
> 403: Key did not match.

---

**Request:**

> `POST /api/{Id}/del` - Delete an event by id.  
> body: `key: Id`

**Errors:**

> 400: Id is invalid base64.  
> 400: key is invalid base64.  
> 404: Event with given id does not exist.  
> 403: Key did not match.

---

**Request:**

> `POST /api/{Id}/user/new` - Add a user to an event.  
> body: `[pass: [u8], User]`

**Errors:**

> 400: Id is invalid base64.  
> 404: Event with given id does not exist.  
> 409: User with given name already exists.  
> 500: Password hash failed.

---

**Request:**

> `POST /api/{Id}/user/edit` - Edit a user in an event.  
> body: `[pass: [u8], User]`

**Errors:**

> 400: Id is invalid base64.  
> 404: Event with given id does not exist.  
> 404: User with given name does not exist.  
> 500: Password hash failed.  
> 403: Password did not match.

---

**Request:**

> `POST /api/{Id}/user/del` - Delete a user from an event.  
> body: `[pass: [u8], name: string]`

**Errors:**

> 400: Id is invalid base64.  
> 404: Event with given id does not exist.  
> 404: User with given name does not exist.  
> 500: Password hash failed.  
> 403: Password did not match.
