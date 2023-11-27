# zallocator
An attempt at bringing Zig like allocators to stable Rust.
It would provide a stable implementation for the `Allocator`
trait which was available in nightly rust and also provide
containers which play nicely with said `Allocator` trait.
It is based on Zig's philosophy of BYOA (bring your own allocator).
The structures which I aim to provide are -
- Vec
- Hashmap
- VecDeque
- BTreeSet
- 
