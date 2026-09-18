# Runtime training with forward-mode duals (no reverse tape).
# Minimize (w - 3)^2 starting at w=0 using dual gradients.

let w = 0.0;
let step = 0;
while step < 15 {
  let d = dual(w);
  let err = d - 3.0;
  let loss = err * err;
  print primal(loss);
  # gradient of loss w.r.t. w is tangent(loss)
  let g = tangent(loss);
  w = w - 0.2 * g;
  step = step + 1;
}
print w;
