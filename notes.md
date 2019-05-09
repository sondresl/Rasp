## Notes on current state of project

### Pretty print / test\_parser
The current implementation of test\_parser is a mix between test\_parser and pretty print from the INF2100-project. The original project creates the logfile during parsing.  Currently, it is created by calling a separate test\_parser functino on the ast.

#### Indentation of test\_parser
During the write to test\_parser output, indents are written first, then the line we want.
I find this to be an ugly solution. Is there maybe a better way to pad the output?

```rust
for _ in 0..=(indentation * 2) { file.write(b" ")?; };
```

### Project structure
I want to look into different wys of structuring the project. It might be good to just stick to the way it wasin 2100,
but there might be a more idiomatic way of doing this in rust. Mainly im thinking about separing implementation blocks
into different modules. For example, the logger module would provide the impl for each ast element with the logging
functions.

### Error handling
The logger currently returns unchecked results because it is writing to file.
ParserError need to implement From<io error> or something.
ParserError also should be an enum with a few variants, and possibly using the failure crate.

## TODO (Not the obvious ones)

 - Rewrite enter\_parser into a higher-order function.

