# Pitfalls of async/await

Async / await provides convenient and efficient abstraction for concurrent asynchronous programming. However, the async/await model in Rust also comes with its share of pitfalls and footguns. We illustrate some of them in this chapter:

---

- [Blocking the executor](pitfalls/blocking-executor.md)
- [Pin](pitfalls/pin.md)
- [Reentering threads](pitfalls/reentering-threads.md)
