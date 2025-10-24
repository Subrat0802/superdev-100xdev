// struct Rect {
//     width: u32,
//     height: u32,
// }

// struct Square {
//     side: u32,
// }

// trait Shape {
//     fn area(&self) -> u32;
//     fn perimeter(&self) -> u32;
// }

// impl Shape for Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn perimeter(&self) -> u32 {
//         2 * (self.height + self.width)
//     }
//     // fn shape() -> String {
//     //     return String::from("Subrat")
//     // }
// }

// impl Shape for Square {
//     fn area(&self) -> u32 {
//         self.side * self.side
//     }

//     fn perimeter(&self) -> u32 {
//         4 * (self.side)
//     }
// }


// fn main() {
//     let rect = Rect {
//         width: 20,
//         height: 30,
//     };

//     let square = Square { side: 20 };

//     // println!("{}", rect.area());
//     // println!("{}", Rect::shape());
//     // println!("{}", square.area());

//     let (area, peri) = get_area_peri(square);
//     println!("{} {}", area, peri);
// }

// fn get_area_peri(s: impl Shape) -> (u32, u32) {
//     return (s.area(), s.perimeter());
// }



//-=======================

// struct User {
//     name: String,
//     age: u32
// }

// fn main() {
//     let u = User {
//         name: String::from("Subrat"),
//         age: 20
//     };

//     println!("{}", u);  //user doesnt implement the display trait error
// }

//------------

// struct User {
//     name: String,
//     age: u32
// }

// impl std::fmt::Display for User {         //add display trait for struct
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "name is {}, age is {}", self.name, self.age)
//     }
// }

// fn main() {
//     let u = User {
//         name: String::from("Subrat"),
//         age: 20
//     };

//     println!("{}", u);  //user doesnt implement the display trait error
// }


//-----------
//now derive macros come into picture because i will not wrote this code again and again it 
//is confusing so here derive macros comes into picture

// #[derive(Debug)]    //debug macro   custom derive macro
// struct User {
//     name: String,
//     age: u32
// }

// fn main() {
//     let u = User {
//         name: String::from("Subrat"),
//         age: 20
//     };

//     println!("{:?}", u);  
// }

//-------------

// #[derive(Debug)]    //debug macro   custom derive macro
// struct User {
//     name: String,
//     age: u32
// }

// fn main() {
//     let u = User {
//         name: String::from("Subrat"),
//         age: 20
//     };

//     println!("{:?}", u);  
// }

//--------

//types of macros

// #[derive(Debug)]    //debug is custum derive peocedural macro
// struct User {
//     name: String,
//     age: u32
// }

// fn main(){
//     println!("Hi there"); //declarative macros

// }

// #[post("/user/")] //attribute like macro
// fn create_user() {
//     sqlx::query!("Insert into user value ()");  //fn like proc macro
// }


//--------

// #[derive(Debug)]
// struct User {
//     name: String,
//     age: u32,
//     address: Address
// }
// #[derive(Debug)]
// struct Address {
//     city: String
// }
// fn main() {
//     let add = Address {
//         city: String::from("Bhopal")
//     };
//     let user = User {
//         name: String::from("Subrat"),
//         age: 28,
//         address: add
//     };

//     println!("{:?}", user);

// }

// // output User { name: "Subrat", age: 28, address: Address { city: "Bhopal" } }







//===============
//Creating our own custom derive macro - Part 1

trait Serialize {
	fn serialize(&self) -> Vec<u8>;
}

trait Deserialize {
	fn desrialize(v: &[u8]) -> Result<Swap, std::fmt::Error> ;
}

#[derive(Debug)]
struct Swap {
    qty_1: u32,
    qty_2: u32
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];
        v.extend_from_slice(&self.qty_1.to_be_bytes());
        v.extend_from_slice(&self.qty_2.to_be_bytes());
        return v;
    }
}

impl Deserialize for Swap {
    fn desrialize(data: &[u8]) -> Result<Self, std::fmt::Error> {
        if data.len() < 8{
            return Err(std::fmt::Error)
        }
        let qty_1 = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let qty_2 = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);

        return Ok(Swap {
            qty_1,
            qty_2
        });
    }
}


fn main() {
    let s = Swap {
        qty_1: 1,
        qty_2: 2
    };

    let s = s.serialize();
    print!("{:?}", s);

    let d = Swap::desrialize(&s).unwrap();
    println!("{:?}", d);


}