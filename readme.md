# :bridge_at_night: fea :globe_with_meridians:

> A simple finite element analysis solver made for educational purposes


Currently the project can only be used to solve problems where all points are constrained
by boundary conditions, but this will be improved in future.
The project also has no GUI for defining the problem at the moment, and must be written
in rust (see [`src/bin/two_d.rs`](./src/bin/two_d.rs) for an example of setting up a problem).
