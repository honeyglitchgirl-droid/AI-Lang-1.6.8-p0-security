# Tiny linear unit trained with dual AD (processor-free)
# Target: y = 3*x on x in {1,2,3}
import "std/core.al";

let w = 0.0;
let lr = 0.1;
let step = 0;

while step < 50 {
  let x = 1.0 + (step - (step / 3) * 3) * 1.0;
  let y_true = 3.0 * x;
  let wd = dual(w);
  let pred = wd * x;
  let err = pred - y_true;
  let loss = err * err;
  let g = tangent(loss);
  w = w - lr * damp(g);
  step = step + 1;
}

print w;
