# rust-integ-test-example

Run with:

```
cargo test -- --nocapture           
```

Output:

```
Initializing shared resources

running 4 tests
test first_tests::first_tests_a ... ok
test first_tests::first_tests_b ... ok
test second_tests::second_tests_b ... ok
test second_tests::second_tests_a ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Dropping shared resources
```