use super::*;
use crate::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct World {
    pub points: HashMap<usize, Rc<RefCell<Point>>>,
    pub beams: HashMap<usize, Rc<RefCell<Beam>>>,
}

impl World {
    pub fn dof(&self) -> usize {
        self.points.len() * 2
    }

    pub fn reduced_dof(&self) -> usize {
        self.reduced_displacement().len()
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.dof(), self.dof())
    }

    pub fn reduced_shape(&self) -> (usize, usize) {
        (self.reduced_dof(), self.reduced_dof())
    }
    /// Create the stiffness matrix for the system of equations to solve
    pub fn stiffness(&self) -> Vec<f32> {
        let dof = self.dof();
        let mut a: Vec<f32> = vec![0.0; dof.pow(2)];

        for point in self.points.values() {
            let point = point.borrow();
            for beam in &point.beams {
                let beam = beam.upgrade().unwrap();
                let beam = beam.borrow();
                let other_point = beam.other_point(&point).unwrap();
                let other_point = other_point.borrow();
                let ax = 2 * (point.id - 1);
                let ay = ax + 1;
                let bx = 2 * (other_point.id - 1);
                let by = bx + 1;

                let angle = beam.angle();
                let c2 = angle.cos().powi(2);
                let s2 = angle.sin().powi(2);
                let cs = angle.cos() * angle.sin();
                let stiffness = beam.stiffness();

                // F_AX += K_AB * c2 * (U_AX)
                a[ax * dof + ax] += stiffness * c2;
                // F_AX += K_AB * cs * U_AY
                a[ax * dof + ay] += stiffness * cs;
                // F_AY += K_AB * cs * U_AX
                a[ay * dof + ax] += stiffness * cs;
                // F_AY += K_AB * s^2 * U_AY
                a[ay * dof + ay] += stiffness * s2;

                // F_AX += K_AB -c^2 U_BX
                a[ax * dof + bx] += stiffness * -c2;
                // F_AX += K_AB -cs U_BY
                a[ax * dof + by] += stiffness * -cs;
                // F_AY += K_AB -cs U_BX
                a[ay * dof + bx] += stiffness * -cs;
                // F_AY += K_AB -s^2 U_BY
                a[ay * dof + by] += stiffness * -s2;
            }
        }

        a
    }
    fn idx_to_coord(&self, idx: usize) -> (usize, usize) {
        let shape = self.shape();

        let row = idx / shape.0;
        let col = idx - (row * shape.0);

        (row, col)
    }

    pub fn reduced_stiffness(&self) -> Vec<f32> {
        let d = self.displacement();

        self.stiffness()
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| {
                let (row, col) = self.idx_to_coord(i);
                //println!("row = {} d[row] = {:?}", row, d[row]);
                //println!("col = {} d[col] = {:?}", col, d[col]);
                d[row] == Value::Unknown && d[col] == Value::Unknown
            })
            .map(|(_, e)| e)
            .collect()
    }

    pub fn displacement(&self) -> Vec<Value> {
        let mut a = vec![Value::Known(0_f32); self.dof()];

        for point in self.points.values() {
            let point = point.borrow();
            let ax = 2 * (point.id - 1);
            let ay = ax + 1;

            if let BoundaryCondition::Fixed = point.bc {
                continue;
            }

            a[ax] = Value::Unknown;
            a[ay] = Value::Unknown;
        }

        a
    }

    pub fn reduced_displacement(&self) -> Vec<Value> {
        self.displacement()
            .into_iter()
            .filter(|d| *d == Value::Unknown)
            .collect::<Vec<_>>()
    }

    pub fn force(&self) -> Vec<Value> {
        let mut a = vec![Value::Unknown; self.dof()];
        for point in self.points.values() {
            let point = point.borrow();
            let ax = 2 * (point.id - 1);
            let ay = ax + 1;

            if let BoundaryCondition::Force(v) = &point.bc {
                a[ax] = Value::Known(v.0);
                a[ay] = Value::Known(v.1);
            }
        }

        a
    }

    pub fn reduced_force(&self) -> Vec<Value> {
        let d = self.displacement();

        self.force()
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| d[i] == Value::Unknown)
            .map(|(_, f)| f)
            .collect()
    }

    pub fn link(&mut self, id1: usize, id2: usize, new_beam: NewBeam) -> Result<(), &str> {
        let b_id = self.beams.len();
        let p1 = match self.points.get(&id1) {
            Some(point) => point,
            None => return Err("Point with id1 not found"),
        };
        let p2 = match self.points.get(&id2) {
            Some(point) => point,
            None => return Err("Point with id1 not found"),
        };

        let b = Rc::new(RefCell::new(Beam::new(
            Rc::downgrade(p1),
            Rc::downgrade(p2),
            new_beam,
        )));

        {
            b.borrow_mut().id = b_id;
        }

        p1.borrow_mut().beams.push(Rc::downgrade(&b));
        p2.borrow_mut().beams.push(Rc::downgrade(&b));

        self.beams.insert(b_id, b);

        Ok(())
    }
}

impl From<Vec<Point>> for World {
    /// Create a world from a list of points.
    /// Expects each point to have a unique ID.
    /// If this is not the case, then some points will be overriden.
    fn from(points: Vec<Point>) -> World {
        let mut points = points;
        let mut w = World {
            points: HashMap::new(),
            beams: HashMap::new(),
        };

        while let Some(point) = points.pop() {
            w.points.insert(point.id, Rc::new(RefCell::new(point)));
        }

        w
    }
}
