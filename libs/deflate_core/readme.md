
## Design Decisions

#### Individual Bit Parsing

I thought of two different approaches here:
1. Create a function that just takes two indices (start and end) of the bits that should be read from the file (multiple bytes)
2. Create a function that maintains an internal state and just takes one input parameter (n) which means read the next n bytes

At first i thought that the 1. approach would be more flexible, but a quick research showed that this is very untypical for a such a use case because the caller of the function will have to maintain the index then. This might lead to mistakes and miss interpretation (especially when having many different variables lengths which are dynamically calculated).
Furthermore the flexibility is not important in this context because every bit sequence is only needed once anyway.
Another argument for the second method is that it enables streaming of data which means that there is no need to load the whole file into memory first. For this project this does not make any difference because this feature is not used currently, but it is still worth mentioning.


