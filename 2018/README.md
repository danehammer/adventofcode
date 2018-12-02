# 2018

Documenting my thoughts as I use these puzzles to learn about Rust.

# Day 1

First part was easy, just google how to read lines of a file, do some arithmetic. Avoided doing two passes.

Part two I got stuck for a while. Adding a second loop around my lines loop caused an ownership problem:

```
error[E0382]: use of moved value: `lines`
  --> src/main.rs:27:23
   |
27 |         for (_, l) in lines {
   |                       ^^^^^ value moved here in previous iteration of loop
   |
   = note: move occurs because `lines` has type `...`, which does not implement the `Copy` trait
```

The gist being, you need to let the for loop borrow the lines, but I couldn't figure out how to do
that with the google-search-result types I had on hand. I first pushed the file reading into the loop,
which I knew meant I re-read and parsed the file each time, lame. I eventually moved the parsing to
a loop that would only run once, saving a vector of the changes. Then the same problem appeared:

```
error[E0382]: use of moved value: `changes`
  --> src/main.rs:27:23
   |
27 |         for change in changes {
   |                       ^^^^^^^ value moved here in previous iteration of loop
   |
   = note: move occurs because `changes` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
```

But that was just changing `for change in changes` to `for change in &changes`.

The other fun thing I learned about was the ability to use the vector's binary search `Result` to either break out and return the solution, or use the `Result` yielding the position the new value should get inserted into. This took away may first approach of re-sorting every push so I could use the binary search instead of going in order thru the vector. Now I insert at the correct location. Timing went from 6-7 seconds to <1 second.