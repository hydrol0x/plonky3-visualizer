use p3_air::{AirBuilder, BaseAir};
use p3_field::{Algebra, Field, PrimeCharacteristicRing};
use p3_matrix::dense::RowMajorMatrix;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::rc::Rc;

#[derive(Clone, Copy, Debug)]
pub struct VisualVar {
    pub col: usize,
    pub row_offset: usize, // 0 = current row, 1 = next row
}

#[derive(Clone, Debug)]
pub enum VisualExpr<F: Field + PrimeCharacteristicRing> {
    Const(F),
    Var(VisualVar),

    Add(Rc<VisualExpr<F>>, Rc<VisualExpr<F>>),
    Sub(Rc<VisualExpr<F>>, Rc<VisualExpr<F>>),
    Mul(Rc<VisualExpr<F>>, Rc<VisualExpr<F>>),
    Neg(Rc<VisualExpr<F>>),
}
trait IntoVisualExpr<F: Field> {
    fn into_expr(self) -> VisualExpr<F>;
}

impl<F: Field> IntoVisualExpr<F> for VisualExpr<F> {
    fn into_expr(self) -> VisualExpr<F> {
        self
    }
}

impl<F: Field> IntoVisualExpr<F> for F {
    fn into_expr(self) -> VisualExpr<F> {
        VisualExpr::Const(self)
    }
}

impl<F: Field> IntoVisualExpr<F> for VisualVar {
    fn into_expr(self) -> VisualExpr<F> {
        VisualExpr::Var(self)
    }
}

impl<F: Field> From<VisualVar> for VisualExpr<F> {
    fn from(v: VisualVar) -> Self {
        VisualExpr::Var(v)
    }
}

impl<F: Field> Add for VisualExpr<F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        VisualExpr::Add(Rc::new(self), Rc::new(rhs))
    }
}

impl<F: Field> Sub for VisualExpr<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        VisualExpr::Sub(Rc::new(self), Rc::new(rhs))
    }
}

impl<F: Field> Mul for VisualExpr<F> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        VisualExpr::Mul(Rc::new(self), Rc::new(rhs))
    }
}

impl<F: Field> Add<F> for VisualExpr<F> {
    // Adding Expr to a elem of field
    type Output = Self;
    fn add(self, rhs: F) -> Self {
        self + VisualExpr::Const(rhs)
    }
}

impl<F: Field> Sub<F> for VisualExpr<F> {
    type Output = Self;
    fn sub(self, rhs: F) -> Self {
        self - VisualExpr::Const(rhs)
    }
}

impl<F: Field> Mul<F> for VisualExpr<F> {
    type Output = Self;
    fn mul(self, rhs: F) -> Self {
        self * VisualExpr::Const(rhs)
    }
}

impl<F: Field> AddAssign<F> for VisualExpr<F> {
    fn add_assign(&mut self, rhs: F) {
        *self = self.clone() + rhs;
    }
}

impl<F: Field> SubAssign<F> for VisualExpr<F> {
    fn sub_assign(&mut self, rhs: F) {
        *self = self.clone() - rhs;
    }
}

impl<F: Field> MulAssign<F> for VisualExpr<F> {
    fn mul_assign(&mut self, rhs: F) {
        *self = self.clone() - rhs;
    }
}

impl<F: Field> From<F> for VisualExpr<F> {
    fn from(value: F) -> Self {
        VisualExpr::Const(value)
    }
}

// // impl<F: Field> PrimeCharacteristicRing for VisualExpr<F> {}
// impl<F: Field> Algebra<F> for VisualExpr<F> {}
//
// pub struct AirVisualizerBuilder<F: Field> {
//     pub constraints: Vec<VisualExpr<F>>,
//     pub width: usize,
// }
//
// impl<F: Field> AirBuilder for AirVisualizerBuilder<F> {
//     type F = F;
//     type Expr = VisualExpr<F>;
// }

