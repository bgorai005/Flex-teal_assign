# Euler Method for IVP in Rust
## Problem Statement
Solve the following Initial Value Problem (IVP) using the Euler method:
```math


\frac{dy}{dt} = \cos(t) - y,\quad y(0) = 1,\quad \text{for } t \in [0, 5]

```
### ✅ Objectives:
- Implement **Euler's method** to approximate the solution numerically.
- Compare the numerical solution against the **exact analytical solution**:
```math

y(t) = \frac{1}{2} \left( \cos(t) + \sin(t) + e^{-t} \right)

```
- Allow the user to choose the number of iterations `n`.
- Save the results to a **CSV file** named based on the iteration count (e.g., `iteration_20.csv`).
- Generate a **plot** comparing the Euler approximation and exact solution, saved as `iteration_20.png` inside the `output/` directory.


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
- Step -1
```
git clone https://github.com/your-username/euler_ivp.git
cd euler_ivp
```
- Step- 2
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

