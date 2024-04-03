# To Run

'cargo run'

Expected to see some output in the console.
The idea is that table is now uninitialized because the show fucntion took ownership of it.

Hashmap is not a Copy trait since it is a dynamically allocated table.

The move semantics has destroyed the entire structure of ownership.

To overcome this issue, there are two options:
1. Shared reference &T
Read only and could share many references. Copy trait
2. Mutable Reference &mut T
Read and write, but can not have other reference active to the value at the same time. Not Copy trait.

> Rule: Multiple readers or single writer

Takeaways:

Allow functions to access or manipulate a structure without taking ownership!


