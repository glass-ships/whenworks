# WhenWorks backend

## Setup

Run the server with `cargo run`

Edit [main.rs](src/main.rs) to change address and port

## Endpoints

#### get the index:  
- request: `GET /` -> index.html

#### get an event:  
- request: `GET /api/<event_id>` -> `{event_entry}`

event_entry
```json
{
    "name": "example event",
    "desc": "this is an example event",
    "creation_date": 1234567890,
    "dates": [
        {
            "from": 1234567890,
            "to": 1234567890,
            "preferred": false
        }
    ],
    "users": {
        "user1": {
            "comment": "this is a comment",
            "avail_dates": [
                {
                    "from": 1234567890,
                    "to": 1234567890,
                    "preferred": false
                },
                {
                    "from": 1234567890,
                    "to": 1234567890,
                    "preferred": true
                }
            ]
        }
    }
}
```

#### create new event:  
- request: `POST /api/new` + `{body}` -> `{event_id, edit_key}`  

body
```json
{
    "name": "example event",
    "desc": "this is an example event",
    "creation_date": 1234567890,
    "dates": [
        {
            "from": 1234567890,
            "to": 1234567890,
            "preferred": false
        }
    ],
    "users": {},
}
```

response
```json
{
    "event_id": "7nd0ws",
    "edit_key": "msjk1G"
}
```

#### check validity of a key:  
- request: `POST /api/<event_id>?<edit_key>` -> `{event_entry}`

event_entry
```json
{
    "name": "example event",
    "desc": "this is an example event",
    "creation_date": 1234567890,
    "dates": [
        {
            "from": 1234567890,
            "to": 1234567890,
            "preferred": false
        }
    ],
    "users": {
        "user1": {
            "comment": "this is a comment",
            "avail_dates": [
                {
                    "from": 1234567890,
                    "to": 1234567890,
                    "preferred": false
                },
                {
                    "from": 1234567890,
                    "to": 1234567890,
                    "preferred": true
                }
            ]
        }
    }
}
```

#### edit an event:  
- request: `POST /api/<event_id>?<edit_key>` + `{body}` -> \_

body
```json
{
    "name": "example event",
    "desc": "this is an example event",
    "dates": [
        {
            "from": 1234567890,
            "to": 1234567890,
            "preferred": false
        }
    ],
    "deleted_users": [
        "user1"
    ],
}
```

#### add a user:
- request: `POST /api/<event_id>/usr` + `{body}` -> \_

body 
```json
{
    "name": "user1",
    "pass": "928344",
    "comment": "this is a comment",
    "avail_dates": [
        {
            "from": 1234567890,
            "to": 1234567890,
            "preferred": false
        },
        {
            "from": 1234567890,
            "to": 1234567890,
            "preferred": true
        }
    ]
}
```

#### edit a user:
- request: `POST /api/<event_id>/usr?e` + `{body}` -> \_

body
```json
{
    "name": "user1",
    "pass": "928344",
    "comment": "this is a comment",
    "avail_dates": [
        {
            "from": 1234567890,
            "to": 1234567890,
            "preferred": false
        },
        {
            "from": 1234567890,
            "to": 1234567890,
            "preferred": true
        }
    ]
}
```

#### delete a user:
- request: `POST /api/<event_id>/usr?d` + `{body}` -> \_

body
```json
{
    "name": "user1",
    "pass": "928344",
}
```
