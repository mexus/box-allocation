# box-large-array

A test application to compare "naïve" box allocation (with a stack copy) and a
maybe-uninit approach.

```
% ulimit -s 1048600
% cargo run --release -- --naive
...
sum = 136905762075
Calculated in 2.084766379s

% cargo run --release
...
sum = 136905762075
Calculated in 1.62903329s
```
