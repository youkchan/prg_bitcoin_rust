#[derive(Debug)]
pub struct FieldElement {
    num: i64,
    prime: i64
}

impl FieldElement {
    pub fn new(num: i64, prime: i64) -> Result<FieldElement, String> {
        if num >= prime || num < 0 {
            return Err("Num must be smaller than 0 and prime".to_string())
        }
        Ok(FieldElement {num, prime})
    }
    
    pub fn equal(&self, other:FieldElement) -> bool {
        return self.num == other.num && self.prime == other.prime
    }
    
    pub fn add(&self, other:FieldElement ) -> Result<FieldElement, String> {
        if self.prime != other.prime {
            return Err("Must be same prime number".to_string())
        }
        let new_num = (self.num + other.num) % self.prime;
        Ok(FieldElement {num: new_num, prime: self.prime})
    }

    pub fn mul(&self, other:FieldElement) -> Result<FieldElement, String> {
        if self.prime != other.prime {
            return Err("Must be same prime number".to_string())
        }
        let new_num = self.num * other.num % self.prime;
        Ok(FieldElement {num: new_num, prime: self.prime})
    }

    pub fn pow(&self, num: i64) -> Result<FieldElement, String> {
        if num == 0 {
            return Ok(FieldElement {num: 1, prime: self.prime})
        } else if num == 1 {
            return Ok(FieldElement {num: self.num, prime: self.prime})
        }

        let mut new_exp: i64 = num;
        if num < 0 {
            new_exp = self.prime - 1 + num
        }

        let mut new_num: i64 = self.num;
        for n in 2..=new_exp {
            new_num = new_num * self.num % self.prime;
        }
        Ok(FieldElement {num: new_num, prime: self.prime})
    }

    pub fn truediv(&self, other:FieldElement) -> Result<FieldElement, String> {
        if self.prime != other.prime {
            return Err("Must be same prime number".to_string())
        }
        let exp: i64 = other.prime - 2;
        let pow_result = other.pow(exp);
        let new_num = self.num * pow_result.unwrap().num % self.prime;
        Ok(FieldElement {num: new_num, prime: self.prime})
    }


}

#[derive(Debug)]
pub struct Point {
    x: Option<i64>,
    y: Option<i64>,
    a: i64,
    b: i64,
}

impl Point {
    pub fn new(x: Option<i64>, y: Option<i64>, a: i64, b: i64) -> Result<Point, String> {
        if x == None && y == None {
            return Ok(Point{x, y, a, b})
        }
        let unwrapped_y = y.unwrap();
        let unwrapped_x = x.unwrap();

        if unwrapped_y.pow(2) != unwrapped_x.pow(3) + a * unwrapped_x + b {
            return Err("not on the curve".to_string())
        }
        Ok(Point{x, y, a, b})
    }

    pub fn equal(&self, other:Point) -> bool {
        return self.a == other.a && self.b == other.b && self.x == other.x && self.y == other.y
    }

    pub fn add(&self, other:Point ) -> Result<Point, String> {
        if self.a != other.a || self.b != other.b {
            return Err("not same curve".to_string())
        }

        if self.x == None {
            return Ok(other)
        }

        if other.x == None {
            return Ok(Point{x: self.x, y: self.y, a: self.a, b: self.b})
        }

        if self.x == other.x && self.y != other.y {
            return Ok(Point{x: None, y: None, a: self.a, b: self.b})
        }

        if self.x != other.x {
            let unwrapped_other_y = other.y.unwrap();
            let unwrapped_self_y = self.y.unwrap();
            let unwrapped_other_x = other.x.unwrap();
            let unwrapped_self_x = self.x.unwrap();

            let s = (unwrapped_other_y - unwrapped_self_y) / (unwrapped_other_x - unwrapped_self_x);
            let new_x = s.pow(2) - unwrapped_self_x - unwrapped_other_x;
            let new_y = s * unwrapped_self_x - s * new_x - unwrapped_self_y;
            return Ok(Point{x: Some(new_x), y: Some(new_y), a: self.a, b: self.b})
        }
        Ok(other)

    }
 
}