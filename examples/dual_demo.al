# Forward-mode dual numbers — first and second order.

# First order
let x = dual(3.0);
print x;
let y = x * x;
print y;
print tangent(y);
print primal(y);

# Second order via dual(dual(...))
let x2 = dual(dual(3.0));
print x2;
let y2 = x2 * x2;   # f=x² → f'=6, f''=2 at x=3
print y2;
print tangent(y2);
print primal(y2);
