# AI-Lang 1.1 — calculator-style arithmetic (pure .al, processor-free)
# cargo run -- examples/calculator.al

let a = 12.0;
let b = 4.0;
print a + b;
print a - b;
print a * b;
print a / b;
print (a + b) * 2.0 - 1.0;

if a > b {
  print 1.0;
} else {
  print 0.0;
}

# Running total with self-assignment (fixed in 1.1)
let total = 0.0;
total = total + 10.0;
total = total + 5.5;
total = total - 2.0;
total = total * 2.0;
print total;

let n = 7;
print n + 3;
print n * n;

let i = 1;
let s = 0;
while i < 6 {
  s = s + i;
  i = i + 1;
}
print s;
