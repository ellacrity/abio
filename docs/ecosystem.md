## Ecosystem
 
 
 The ecoysystem that attempts to solve the problem of how to perform direct, yet safe, reinterpretation of raw bytes into concrete types is somewhat small. There are not many crates that allow you to work with bytes at this level, and they all mostly work the same.
 
 They place heavy restrictions on the way your types must be represented in memory and you, as an end-user, must accept the contract introduced by the crate in order to write sound code.
 
 There is no avoiding that because the libraries that work this way must enforce certain contracts with regard to the layout of your types to produce sound implementations. By placing restrictions on what you **can** do, we allow you to perform an operation that you typically "**should not be able to**". The reason behind this is, for the most part, because this is extremely unsafe code that must be written properly and correctly to work as intended. 

### How other libraries work

Most, if not all, "zero-copy" or "safe transmute" crates use a combination of
[`ptr::read_unaligned`][`core::ptr::read_unaligned`] and the `*` dereference pattern to
dereference a raw pointer and take a reference to the data in one fell swoop. This is a valid technique and it works fairly well.
