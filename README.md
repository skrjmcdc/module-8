# Reflection

## 1. Unary vs. server vs. bidirectional streaming

Unary methods work just like "regular" request-response models. The client calls a method, which sends one HTTP request to the server, and the server then sends one HTTP response back to the client. The main difference is that the API through which the client communicates with the server takes the form of a remote method that the client can call as though it were a local method, whereas in the "regular" request-response model the client explicitly sends a request to the server, and explicitly waits for a response from the server. This is useful anywhere you would use the standard request-response model.

Like a unary method, in a server streaming method the client (indirectly) sends a request to the server. *Unlike* a unary method, the server doesn't just send one response back to the client, but a stream of responses. This is useful when the client needs constant data from the server, such as weather updates or video livestreams.

In a bidirectional streaming method, *both* the server and the client communicate in streams of messages. This is best suited when both the client and the server need constant updates from the other side, for example in chat applications or drones.

## 2

## 3

## 4

## 5

I would separate both the server's and the client's code into separate .rs files.

For the server, I could put all the services in separate files in a service directory. So in this example I would have `service/payment_service.rs`, `service/transaction_service.rs`, and `service/chat_service.rs` files.

As for the client, I could again put all the client code in separate files, so I would have `client/payment_client.rs`, `client/transaction_client.rs`, and `client/chat_client.rs` files.

## 6

* The server might enforce that the amount be positive.
* The server might validate the user id first.

## 7

## 8

## 9



## 10
