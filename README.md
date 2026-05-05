# Reflection

## 1. Unary vs. server vs. bidirectional streaming

Unary methods work just like "regular" request-response models. The client calls a method, which sends one HTTP request to the server, and the server then sends one HTTP response back to the client. The main difference is that the API through which the client communicates with the server takes the form of a remote method that the client can call as though it were a local method, whereas in the "regular" request-response model the client explicitly sends a request to the server, and explicitly waits for a response from the server. This is useful anywhere you would use the standard request-response model.

Like a unary method, in a server streaming method the client (indirectly) sends a request to the server. *Unlike* a unary method, the server doesn't just send one response back to the client, but a stream of responses. This is useful when the client needs constant data from the server, such as weather updates or video livestreams.

In a bidirectional streaming method, *both* the server and the client communicate in streams of messages. This is best suited when both the client and the server need constant updates from the other side, for example in chat applications or drones.

## 2

## 3

## 4

## 5

## 6

## 7

## 8

## 9



## 10
