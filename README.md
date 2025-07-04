# Euler Method for IVP in Rust
## Problem Statement
Solve the following Initial Value Problem (IVP) using the Euler method:

𝑑
𝑦
𝑑
𝑡
=
cos
⁡
(
𝑡
)
−
𝑦
,
𝑦
(
0
)
=
1
,
0
≤
𝑡
≤
5
dt
dy
​
 =cos(t)−y,y(0)=1,0≤t≤5
Approximate the solution using Euler’s method for a user-defined number of iterations n.

Compare it with the analytical solution:

𝑦
(
𝑡
)
=
1
2
(
cos
⁡
(
𝑡
)
+
sin
⁡
(
𝑡
)
+
𝑒
−
𝑡
)
y(t)= 
2
1
​
 (cos(t)+sin(t)+e 
−t
 )
Export results to CSV

Generate a plot using the plotters crate.

## Project Structure

```
euler_ivp/
├── Cargo.toml
├── output/
│   ├── euler_solution.csv      # CSV output file
│   └── plot.png                # Plot comparing Euler and exact solution
└── src/
    ├── main.rs                 # Entry point: handles I/O and workflow
    ├── solver.rs               # Euler method implementation
    ├── exact.rs                # Exact analytical solution
    ├── utils.rs                # CSV export utility
    └── plot.rs                 # Generates PNG plot using Plotters
```

---
## Requirements
[Rust (latest stable)](https://www.rust-lang.org/tools/install)


##  How to Run
- Step 1
```
git clone https://github.com/your-username/euler_ivp.git
cd euler_ivp
```
-step 2
```
cargo build
```
- Step -3
```
cargo run
```
- Step -4
 enter the number of iterations n. Example:
 ```
 Enter the number of iterations (n):
100
```

