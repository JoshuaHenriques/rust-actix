# README
Webserver demo to learn Rust and actix. Includes web sockets.

## Websockets
Websockets in actix web heavily use the actor framework.

Each socket is an actor.

1. Messages
2. Actors "Mailboxes"

Actors are entirely independent of eachother.

## todo
* change the endpoint to "/chatroom" and generate a uuid as the roomId
