use specs::prelude::*;
use cgmath::{Matrix4, SquareMatrix};
use std::ops::Deref;

#[derive(Component, Clone)]
pub struct Transform {
    pub matrix: Matrix4<f64>,
    pub inverse: Matrix4<f64>,
}

impl Transform {
    pub fn new(matrix: Matrix4<f64>) -> Self {
        let inverse = matrix.invert().expect("Failed to invert matrix");

        Self { matrix, inverse }
    }
}

impl Deref for Transform {
    type Target = Matrix4<f64>;

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

#[cfg(test)]
mod test;