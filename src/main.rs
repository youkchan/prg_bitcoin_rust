mod ecc;

fn main() {
   
    //let a = ecc::FieldElement::new(3, 10);
    let a = ecc::FieldElement::new(7, 13);
    let b = ecc::FieldElement::new(3, 10);
    let c = ecc::FieldElement::new(4, 10);
    let d = ecc::FieldElement::new(9, 10);
    let e = ecc::FieldElement::new(7, 19);
    let f = ecc::FieldElement::new(5, 19);
    let point = ecc::Point::new(Some(-1), Some(-1), 5, 7);
    let point2 = ecc::Point::new(None, None, 5, 7);
    let point3 = ecc::Point::new(Some(-1), Some(1), 5, 7);
    let point4 = ecc::Point::new(Some(2), Some(5), 5, 7);
    //let point2 = ecc::Point::new(-1, -2, Some(5), Some(7));
    if a.is_ok() && b.is_ok() && c.is_ok() {
        let unwrap_a = a.unwrap();
        let unwrap_point = point.unwrap();
        let unwrap_point2 = point2.unwrap();
        let unwrap_point3 = point3.unwrap();
        let unwrap_point4 = point4.unwrap();
        //println!("{:?}", unwrap_point2.add(unwrap_point));
        println!("{:?}", unwrap_point4.add(unwrap_point));
        //println!("{:?}", unwrap_point.add(unwrap_point3));
        //let unwrap_point2 = point2.unwrap();
        //println!("{:?}", unwrap_a.equal(b.unwrap()));
        //println!("{:?}", a.as_ref().unwrap().equal(b.unwrap()));
        //println!("{:?}", unwrap_a.equal(c.unwrap()));
        //println!("{:?}", a.as_ref().unwrap().equal(c.unwrap()));
        //println!("{:?}", unwrap_a.add(d.unwrap()));
        //println!("{:?}", unwrap_a.mul(d.unwrap()));
        println!("{:?}", unwrap_a.pow(-3));
        println!("{:?}", unwrap_a.pow(3));
        println!("{:?}", e.unwrap().truediv(f.unwrap()));
        //println!("{:?}", unwrap_a.pow(2));
        //println!("{:?}", unwrap_a.pow(3));
        //println!("{:?}", unwrap_a.pow(0));
//        println!("{:?}", unwrap_a.pow(0));

    }
    //println!("{:?}", a.equal(b));
    //println!("{:?}", a.equal(c));
}

