# Background
Websockets in actix web heavily use the actor framework.

Each socket is an actor.

1. Messages
2. Actors "Mailboxes"

Actors are entirely independent of eachother.
