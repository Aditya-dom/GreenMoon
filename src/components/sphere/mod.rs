/* The origin/radius of the sphere can be determined
by the Transform component.
 */

use specs::prelude::*;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Sphere;
 
impl Sphere {
    pub fn new() -> Self {
        Self { }
    }
}