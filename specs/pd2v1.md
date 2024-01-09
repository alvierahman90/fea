# 2D Problem Definition Version 1 (PD2v1) Specification

A format for defining shapes that humans can write and machines can read.

I think it should have a better name.

## Usage

### Beam Elements (BE)

Beam element problem definitions use points to represent nodes and links to represent
beams.

A given point (node) can be connected to any number of links (beams).

### Finite Elements (FE)

Finite element problem definitions use points to represent nodes and links to represent
edges.

A given point (node) can only be connected to two links (edges).

### Mesh

The spec can also be used to export meshes by using fixed boundary conditions and definining a set
of `LINK_PROPS` that can be ignored (i.e. `LINK_PROPS ignore 0 0`).

## Types

### REAL

A real number

#### Examples

```
0
-1
1
-1.0
1.0
9999999.999999
```

### IDs (`ID`)

String that can include numbers, letters, underscores (`_`), and dashes (`-`).

### Boundary condition types (`BC_TYPE`)

- `NONE`
- `FIXED`
- `FORCE <VEC_ID>`

## Syntax

### Comments

```
// comments begin with // and end at the end of a line
```

### Definitions

Each definiton goes on its own line.
Definitons cannot refer to IDs of objects that are defined in latter lines.

Every ID given for every object must be unqiue amongst its object type but not between types (although it is fine if it is).
All definitions are immutable.

| Object Type        | Syntax                                                   |
|--------------------|----------------------------------------------------------|
| Boundary Condition | `BOUNDARY_CONDITION <ID ID> <BC_TYPE BC_TYPE>`           |
| Link               | `LINK <ID ID> <ID LINK_PROPS> <ID NODE_ID> <ID NODE_ID>` |
| Link Properties    | `LINK_PROPS <ID ID> <ID MATERIAL> <REAL AREA>`           |
| Material           | `MATERIAL <ID ID> <REAL YOUNGS_MODULUS>`                 |
| Point              | `POINT <ID ID> <ID VEC_ID> <ID BOUNDARY_CONDITION>`      |
| Vector             | `VEC <ID ID> <REAL X> <REAL Y>`                          |


## Example

### Beam Element

```
// Define a fixed boundary conditon as it can be shared by all fixed points
BOUNDARY_CONDITION bc_fixed FIXED
MATERIAL aluminium 210000000000 250000000
LINK_PROPS beam_props aluminium 0.0019635

// Define the first node
VEC point1_pos 0 0
POINT point1 point1_pos bc_fixed

// Define node with force boundary condition
VEC point4_pos 1.7321 1
VEC bc4_mag 14142 14142
BOUNDARY_CONDITION bc4 FORCE bc4_mag
POINT point4 point4_pos bc4

// Define rest of the nodes
VEC point2_pos 0 1.1547
POINT point2 point2_pos bc_fixed
VEC point3_pos 0 4.4641
POINT point3 point3_pos bc_fixed

// And finally link the nodes together to create the beams
LINK link1 beam_props point1 point4
LINK link2 beam_props point2 point4
LINK link3 beam_props point3 point4
```
