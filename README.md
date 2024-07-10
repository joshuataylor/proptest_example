```shell
# just recopy the example file to the test files, named .txt to ensure no cargo formatting.
cp proptest_formatting_example.txt tests/proptest_formatting_cargofmt_test.rs
cp proptest_formatting_example.txt tests/proptest_formatting_jetbrains_test.rs
cp proptest_formatting_example.txt tests/proptest_formatting_test.rs
cp proptest_formatting_example.txt tests/proptest_test.rs

cargo fmt
```