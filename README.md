# Euler Method for IVP in Rust
## Problem Statement
Solve the following Initial Value Problem (IVP) using the Euler method:
```math


\frac{dy}{dt} = \cos(t) - y,\quad y(0) = 1,\quad \text{for } t \in [0, 5]

```
###  Objectives:
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
│   ├── iteration.csv      # CSV output file
│   └── iteration.png                # Plot comparing Euler and exact solution
└── src/
    ├── main.rs                 # Entry point: handles I/O and workflow
    ├── solver.rs               # Euler method implementation
    ├── exact.rs                # Exact analytical solution
    ├── utils.rs                # CSV export utility
    └── plot.rs                 # Generates PNG plot using Plotters
```

---

## Parameters

This project allows the user to configure the following parameters at runtime or via code:

| Parameter | Type   | Description |
|-----------|--------|-------------|
| `n`       | usize  | Number of iterations (or time steps) used in Euler’s method. This determines the step size `h = (t_end - t_0) / n`. |
| `t_0`     | f64    | Initial time value. For this problem, it is set to 0.0. |
| `t_end`   | f64    | Final time value. For this problem, it is set to 5.0. |
| `y_0`     | f64    | Initial value of the dependent variable `y`. Given as 1.0 in this problem. |
| `f(t, y)` | fn     | The function representing the differential equation. In this case, the ODE is \( y' = \cos(t) - y \). |
| `exact_solution(t)` | fn | The analytical solution function \( y(t) = 0.5(\cos(t) + \sin(t) + e^{-t}) \), used for error comparison. |
| `output_path` | String | The directory where CSV and plot files will be saved. Default: `./output/`. |
| `filename_prefix` | String | Output file name will be `iteration_{n}.csv` and `iteration_{n}.png` where `{n}` is the value entered by the user. |

----

## Requirements
[Rust (latest stable)](https://www.rust-lang.org/tools/install)


##  Follow these steps to build and execute the project:
### Step -1:Clone the Repository
```
git clone https://github.com/your-username/euler_ivp.git
cd euler_ivp
```
### Step- 2:Build the Project
```
cargo build
```
### Step -3:Run the Program
```
cargo run
```
### Step -4:
 enter the number of iterations n. Example:
 ```
 Enter the number of iterations (n):
100
```
## References


- Rust Programming Language: [https://www.rust-lang.org](https://www.rust-lang.org)  
  Official Rust documentation, syntax guide, and installation.

- Plotters Crate Documentation: [https://docs.rs/plotters](https://docs.rs/plotters)  
   Documentation for the Rust plotting library used in this project.

- CSV Crate Documentation: [https://docs.rs/csv](https://docs.rs/csv)  
  Official docs for Rust's CSV reading/writing library.



## Author
Biswajit Gorai

M.Tech IMSC

Department of Mathematics, IIT Madras
