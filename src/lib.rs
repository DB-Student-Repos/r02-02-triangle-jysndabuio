pub struct Triangle{
    x:u64,
    y:u64,
    z:u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let x = sides[0];
        let y = sides[1];
        let z = sides[2];

        if sides.iter().all(|&c| c > 0) && x +y >= z && x + z >= y && y + z >= x {
            Some(Triangle{x, y, z})
        } else {
            None
        }
        // if x > 0 && y > 0 && z > 0 && (x +y) >= z && (x + z) >= y && (y + z) >= x {

    }

    pub fn is_equilateral(&self) -> bool {
        self.x == self.y && self.x == self.z
    }

    pub fn is_scalene(&self) -> bool {
        self.x != self.y && self.x != self.z && self.y != self.z
    }

    pub fn is_isosceles(&self) -> bool {
        !self.is_equilateral() && !self.is_scalene()
    }
}
