## Notes on current state of project

### Pretty print / test_parser
The current implementation of test_parser is a mix between test_parser and pretty print from the INF2100-project. The original project creates the logfile during parsing.  Currently, it is created by calling a separate test_parser functino on the ast.

#### Indentation of test_parser
During the write to test_parser output, indents are written first, then the line we want.
I find this to be an ugly solution. Is there maybe a better way to pad the output?

```rust
for _ in 0..=(indentation * 2) { file.write(b" ")?; };
```

