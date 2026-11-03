# Example Task

As a motivating example, let's look at a web server application that needs to
communicate with both a client and a database.

```bob
Client               Web server               Database
   |                      |                       |
   |       Connect        |                       |
   |--------------------->|                       |
   |                      |                       |
   |       Request        |                       |
   |--------------------->|                       |
   |                      |                       |
   |                      |   Database request    |
   |                      |---------------------->|
   |                      |                       |
   |                      |   Database response   |
   |                      |<----------------------|
   |                      |                       |
   |       Response       |                       |
   |<---------------------|                       |
   |                      |                       |
```

<details>

- In this example we have a web server that accepts a request from a client,
  requests data from a database, and then responds to the client.

- At multiple points while performing this task the server spends time waiting
  on the network:

  - After the client connects, we have to wait for the request to finish
    arriving.
  - We need to send the request to the database.
  - We need to wait for the database's response.
  - We need to send the response to the client.

- This time spent waiting may account for a large portion of the time between
  when the task starts to when it ends: The server may only spend microseconds
  executing its business logic, while it may spend milliseconds waiting on the
  network.

</details>
