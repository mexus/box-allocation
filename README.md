# box-large-array

A test application to compare "naïve" box allocation (with a stack copy) and a
maybe-uninit approach.

```
% ulimit -s 1048600

% cargo run --release -- --naive
sum = 136905762075
Calculated in 1.098912998s

% cargo run --release
...
sum = 136905762075
Calculated in 612.201882ms

% cargo run --release -- --not-that-naive
...
sum = 136905762075
Calculated in 710.362643ms
```

Criterion comparison (on 2 * 1024 * 1024 = 2097152 elements):

```
init/naïve             time:   [163.90 us 164.06 us 164.25 us]
init/maybe-uninit       time:   [60.739 us 60.748 us 60.757 us]
init/not-that-naïve    time:   [105.66 us 106.21 us 106.63 us]
```
