# Elliptic_Curve_Point_Arithmetic_with_Real_Numbers

A simple Rust program to perform elliptic curve point addition and point doubling using real numbers on a simple curve equation.

## Description

This program demonstrates basic elliptic curve arithmetic using real numbers. The main operations implemented include:
- **Point Addition**: Computes the addition of two distinct points on an elliptic curve.
- **Point Doubling**: Computes the doubling of a point on an elliptic curve.

## Code Structure

The code consists of:
- A `Point` struct to represent points on the curve.
- Functions for adding and doubling points on an elliptic curve.

### `Point` Struct
The `Point` struct represents a point with `x` and `y` coordinates of type `f64` (floating-point).

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
```
- `add_points` Function: This function adds two points `p` and `q` using the formula for elliptic curve point
- `double_point` Function: This function performs the point doubling operation for a given point `p`.
- `main` Function: The `main` function demonstrates the usage of `add_points` and `double_point`.
## Usage
- To get started, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine. You can then clone the repository and build the project:
- Clone the repository:
```
git clone https://github.com/cypriansakwa/Elliptic_Curve_Point_Arithmetic_with_Real_Numbers.git
cd Elliptic_Curve_Point_Arithmetic_with_Real_Numbers
```
- Compile and run the program:
```
cargo run
```
