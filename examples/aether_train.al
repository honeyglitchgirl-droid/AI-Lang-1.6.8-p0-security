# AI-Lang invented training pattern using Aether activation + pulse.
import "std/core.al";

let w = 0.0 grad;
let step = 0;
while step < 8 {
  let pred = aether(w);
  let err = pred - 1.5;
  let loss = err * err + pulse(w);
  print loss;
  backward_pass();
  gradient_update(w, 0.3);
  step = step + 1;
}
print w;
