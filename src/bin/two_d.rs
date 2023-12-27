use fea::two_d::beam_elements::*;
use fea::two_d::*;
use fea::Value;
use ndarray::prelude::*;
use ndarray_linalg::Solve;

fn main() {
    let student_id_last_digit = 9_f32;
    let d = student_id_last_digit / 100.0;
    let r = d / 2.0;
    let e = 210e9;
    let p = (25.0 - student_id_last_digit) * 1000.0;
    let theta = 30.0_f32.to_radians();
    let l = 2.0 / theta.cos();

    let p1 = Point {
        id: 1,
        pos: Vector(0.0, 0.0),
        bc: BoundaryCondition::Fixed,
        beams: vec![],
    };

    let p2 = Point {
        id: 2,
        pos: Vector(0.0, l * theta.sin()),
        bc: BoundaryCondition::Fixed,
        beams: vec![],
    };

    let l_e3 = 2.0 / theta.sin();

    let p3 = Point {
        id: 3,
        pos: Vector(0.0, (l * theta.sin()) + (l_e3 * theta.cos())),
        bc: BoundaryCondition::Fixed,
        beams: vec![],
    };

    let p4 = Point {
        id: 4,
        pos: Vector(l * theta.cos(), l * theta.sin()),
        bc: BoundaryCondition::Force(Vector::from_mag_angle(p, 45_f32.to_radians())),
        beams: vec![],
    };

    let mut w = World::from(vec![p1, p2, p3, p4]);

    w.link(
        1,
        4,
        NewBeam {
            cross_section: CrossSection::Circular(r),
            material: fea::Material {
                yield_stress: 95e6,
                youngs_modulus: e,
            },
        },
    )
    .unwrap();
    w.link(
        2,
        4,
        NewBeam {
            cross_section: CrossSection::Circular(r),
            material: fea::Material {
                yield_stress: 95e6,
                youngs_modulus: e,
            },
        },
    )
    .unwrap();
    w.link(
        3,
        4,
        NewBeam {
            cross_section: CrossSection::Circular(r),
            material: fea::Material {
                yield_stress: 95e6,
                youngs_modulus: e,
            },
        },
    )
    .unwrap();

    println!("w = {:?}", w);
    println!("w.shape() = {:?}", w.shape());

    let stiffness = Array2::from_shape_vec(w.shape(), w.stiffness()).unwrap();
    println!("w.stiffness = \n{:?}", stiffness);
    println!("w.displacement() = {:?}", w.displacement());
    println!("w.force() = {:?}", w.force());

    let reduced_stiffness =
        Array2::from_shape_vec(w.reduced_shape(), w.reduced_stiffness()).unwrap();
    let reduced_force = w
        .reduced_force()
        .into_iter()
        .map(|v| match v {
            Value::Known(v) => v,
            Value::Unknown => panic!("Unknown force at unknown vector!"),
        })
        .collect();
    let reduced_force = Array1::from_shape_vec(w.reduced_dof(), reduced_force).unwrap();
    let reduced_displacement =
        Array2::from_shape_vec((1, w.reduced_dof()), w.reduced_displacement()).unwrap();
    println!("w.reduced_stiffness() = \n{:?}", reduced_stiffness);
    println!("w.reduced_force() = {:?}", reduced_force);
    println!("w.reduced_displacement() = {:?}", reduced_displacement);

    println!(
        "soln: {:?}",
        reduced_stiffness.solve_into(reduced_force).unwrap(),
    )
}
