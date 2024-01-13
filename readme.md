# :bridge_at_night: fea :globe_with_meridians:

> A simple finite element analysis solver made for educational purposes


Currently the project can only be used to solve beam element problems where all points are constrained
by boundary conditions, but this will be improved in future.
The project also has no GUI for defining the problem at the moment, and must be written
in rust (see [`src/bin/two_d.rs`](./src/bin/two_d.rs) for an example of setting up a problem).

## Requirements

- [OpenBLAS](https://github.com/OpenMathLib/OpenBLAS)
- A FORTRAN compiler, such as [GFortran](https://gcc.gnu.org/fortran/)

To install on an Arch based distro:

```
pacman -S blas-openblas gcc-fortran
```

# Format Specifications

The project will likely be composed of multiple independent programs as it is developed.
The formats specified in the [specs folder](./specs) will be used to to exchange information between
the programs.
