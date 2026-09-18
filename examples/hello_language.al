# Runnable language smoke test (processor-free)
print "AI-Lang is runnable";
let a = 10;
let b = 3;
print a + b;
print a * b;
if a > b {
  print "a is greater";
} else {
  print "b is greater";
}
let i = 0;
let sum = 0;
while i < 5 {
  sum = sum + i;
  i = i + 1;
}
print sum;
print "done";
