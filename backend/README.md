`cargo run` to run er
edit [main.rs](src/main.rs) to change address and port

# Handled Requests

get the index:  
`GET /`  ->  index.html

get an event:  
`GET /api/<event_id>`  ->  `{event_entry}`

create new event:  
`POST /api/new` + `{body}`  ->  `{event_id, edit_key}`

check validity of a key:  
`POST /api/<event_id>?<edit_key>`  ->  `{event_entry}`

edit an event:  
`POST /api/<event_id>?<edit_key>` + `{body}`  ->  _

add a user  
`POST /api/<event_id>/usr` + `{body}`  ->  _

edit a user  
`POST /api/<event_id>/usr?e` + `{body}`  ->  _
