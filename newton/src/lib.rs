/* vector definitions */
// TODO: change to integers

pub mod vector;

use vector::*;
pub struct Object {
    pub pos: Pos3d,
    pub vel: Vel3d,
    pub mass: f64,
    pub radius: f64,
}
impl Object {
    pub fn new() -> Object {
        let pos = Pos3d( 0.0, 0.0, 0.0 );
        let vel = Vel3d( 0.0, 0.0, 0.0 );
        return Object {
            pos,
            vel,
            mass: 0.0,
            radius: 0.0,
        }
    }
}