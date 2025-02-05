# Unsafe Rust Pointer Invalidation
This repository demonstrates a common error in Rust: using raw pointers to modify a vector that might reallocate its internal data. The consequence of this error can be undefined behavior and a program crash.

The `bug.rs` file contains the erroneous code.  The `bugSolution.rs` file showcases a safer approach, avoiding raw pointers and maintaining memory safety.