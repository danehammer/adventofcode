# 2018

Documenting my thoughts as I use these puzzles to learn about Rust.

### Day 1

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

### Day 2

Part 1 was mostly following the Rust book's info on HashMap. Borrowing ownership feels VERY weird.

Part 2, holy cow. Rust is Scala and Erlang and Haskell and Clojure and C++ all rolled up into one.

### Day 3

Part 1:
And then it hits me. I hate this feeling of not knowing how to do anything with anything in this language. My code never compiles for a bunch of rules I haven't learned about yet. My code is
verbose and I'm sure there's all kinds of helpers to make it more elegant and faster. Ugh. Hate
this part of learning a new language.

Part 2:
It feels like higher level languages do all this memory copying and references for me. Because there's
no GC, I have to care about it. But it sure seems tedious with the ownership compilation failures.