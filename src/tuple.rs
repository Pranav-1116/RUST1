pub fn run(){

     //SIMPLE TUPLE STRUCT 

     // Definition
struct Color(u8, u8, u8);

 {
    // Creation
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);
    
    // Access using dot notation with index
    println!("Red component: {}", red.0);
    println!("Green component: {}", green.1);
    println!("Blue component: {}", blue.2);
    
    // Destructuring
    let Color(r, g, b) = red;
    println!("RGB: ({}, {}, {})", r, g, b);
}

 //TUPLE STRUCTS WITH METHODS

 struct Point3D(f64, f64, f64);

impl Point3D {
    // Associated function (constructor)
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D(x, y, z)
    }
    
    // Method to calculate distance from origin
    fn distance_from_origin(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
    
    // Method to scale the point
    fn scale(&mut self, factor: f64) {
        self.0 *= factor;
        self.1 *= factor;
        self.2 *= factor;
    }
    

    // Method for dot product
    fn dot(&self, other: &Point3D) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

{
    let mut point = Point3D::new(3.0, 4.0, 0.0);
    println!("Distance from origin: {}", point.distance_from_origin());
    
    point.scale(3.0);
    println!("After scaling: ({}, {}, {})", point.0, point.1, point.2);
    
    let other = Point3D::new(1.0, 0.0, 0.0);
    println!("Dot product: {}", point.dot(&other));
}

}