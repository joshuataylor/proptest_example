# Rust proptest format example repo for rustfmt + JetBrains RustRover

## Resetting files
### Option 1: git checkout

Resets all files in the tests/ directory, overwriting any changes made to them:
```shell
git checkout main -- tests/
```

### Option 2: cp from example file
Just recopy the example file to the test files, named .txt to ensure no cargo formatting.

```shell
cp proptest_formatting_example.txt tests/proptest_formatting_cargofmt_test.rs
cp proptest_formatting_example.txt tests/proptest_formatting_jetbrains_test.rs
cp proptest_formatting_example.txt tests/proptest_formatting_test.rs
cp proptest_formatting_example.txt tests/proptest_test.rs

cargo fmt
```

## JetBrains Rust plugin
JetBrains offers a CLI tool to format files, [docs](https://www.jetbrains.com/help/idea/command-line-formatter.html).

MacOS: `RustRover.app/Contents/bin/format.sh -allowDefaults tests/proptest_formatting_jetbrains_test.rs`
Linux: `rustrover/bin/format.sh -allowDefaults tests/proptest_formatting_jetbrains_test.rs`