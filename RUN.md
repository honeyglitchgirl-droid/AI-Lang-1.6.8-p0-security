# Running **your** AI-Lang programs

## Quickest (one-liner)

```bash
ai-lang -e 'print 1 + 2;'
ai-lang -e 'let xs = [10, 20]; print xs[1];'
ai-lang -e 'let i = 3; while i { print i; i = i - 1; }'
```

`-e` / `--eval` / `-c` all work. Missing final `;` is added for you.

## From a file

```bash
# my_prog.al
let xs = [1, 2, 3];
print list_len(xs);
print xs[0];

ai-lang my_prog.al
```

## From stdin (pipes / online paste)

```bash
echo 'print 99;' | ai-lang -
```

## Interactive shell

```bash
ai-lang          # or: ai-lang --repl
ai-lang> print 1+2;
3
ai-lang> :example
ai-lang> :q
```

Multi-line: type several lines, then a blank line to run.

## Build once

```bash
cargo build --release
./target/release/ai-lang -e 'print 42;'
```
