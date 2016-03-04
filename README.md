# codeplatter

codeplatter is a little tool I wrote to compile code snippets for a presentation.

When launched, you send your code via POST request to `localhost:3000/<lang>` and codeplatter will send you back the output.

Works currently with:

- Rust: `localhost:3000/rust`
- C++: `localhost:3000/cpp`
