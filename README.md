# chat-pubsub
Chat client using [publish subscribe pattern](https://en.wikipedia.org/wiki/Publish%E2%80%93subscribe_pattern) (Refactor of https://github.com/arpith/chat-server)

The clients, upon connecting, will subscribe to the message bus. New messages are put on the bus. 
A logging utility subscribes to the message bus and saves all messages to file.
