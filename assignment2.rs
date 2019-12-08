//Q1.Calculator.
// use std::io;
// #[allow(unused_variables)]

// fn main() {
//   let mut input = String::new(); 
//   io::stdin().read_line(&mut input).expect("unable to read line");
//   let x = input.trim();
//   let value:String = x.split_whitespace().collect(); // removes white spaces
//   let mut items: Vec<_> = value.split("").collect(); // splits String to Vector 
//   items.remove(0); // removes starting index containing white space
//   items.remove(items.len() - 1);  // removes ending index containing white space

//   let mut number = String::new();
//   let mut new_items = vec![];

//   for s in items {
//       if s == "+" {
//           // if found an operator, push and clear the current operand and operator
//           new_items.push(number.drain(..).collect());
//           new_items.push(s.to_owned());
//       } else {
//           number.push_str(s);
//       }
//   }

//   if !number.is_empty() {
//       new_items.push(number);
//   }

//   let f_operand:f32 = new_items[0].parse().unwrap(); // parse String to Integer
//   let l_operand:f32 = new_items[2].parse().unwrap(); // parse String to Integer
//   let operator = &new_items[1];


//   if operator == "+" {
//       println!("{}",f_operand + l_operand);
//   }
//   else if operator == "-" {
//       println!("{}",f_operand - l_operand);
//   }
//   else if operator == "*" {
//       println!("{}",f_operand * l_operand);
//   }
//   else if operator == "/" {
//       println!("{}",f_operand / l_operand);
//   }
//   else if operator == "-" {
//       println!("10 - 5 = {}",10-5);
//   }
//   else if operator == "^" {
//       let mut exp = f_operand;
//       for i in 1..(l_operand as i32) {
//           exp = exp * f_operand;   
//       }
//       println!("{}",exp);
//   }

// }
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Q2.Complete the file closures_properties.rs in drive.
// fn main() {
//     let x = //make a closure which takes no argument and prints hello world
//     x();
// }
// fn main() {
//     let x = //Make a closure which takes one u32 data type as argument and returns with adding 1 to it.
//     y = 
//     println!("The function returns: {}",x(y)); 
// }
// fn main() {
//     let c = 1;
//     let x = //Make a closure which captures value of variable "c" from environment and change the value of c with adding 1.
//     x();
//     println!("The new value of c is: {}",c); // should print 2
// }
// // Write a function which accepts a closure, and in the funciton body, it calls the closure. The closure accepts no argument and returns nothing. What should the closure be about? Be creative!
// fn main() {
//     fn main() {
//     let x = || {};
//     let nothing = |x| {x};
//     nothing(x);
// }
// // Write a function which expects a closure as an argument and in the funciton body, it calls the closure. The closure expects u32 argument and returns the u32 value. The closure adds 1 to the argument and returns it.
// fn main() {
//     let num = 2;
//     let x = |num :u32| -> u32 { num+1};
//     let y = x(num);
//     println!("{}",y);
//     }
// }
*/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Q3.
// #[derive(Debug)]
// struct Teacher<T> where T: Fn(u16) -> u16
// {
//     student_fees: T,
//     value: Option<u16>,
// }

// impl<T> Teacher<T>  where T: Fn(u16) -> u16
// {
//     fn new(student_fees: T) -> Teacher<T> {
//         Teacher {
//             student_fees,
//             value: None,
//         }
//     }
//     fn value(&mut self, arg: u16) -> u16 {
//         match self.value {
//             Some(v) => {
//               v
//             } ,
//             None => {
//                 let v = (self.student_fees)(arg);
//                 self.value = Some(v);
//                 v
//             },
//         }

//     }
// }
// fn main() {
//   let mut teacher = Teacher::new(|y| y);
//   teacher.value(2000);
//   println!("Student fees / value : {:#?}",teacher.value.unwrap());
// }


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Q4.

// #[derive(Debug)]
//  struct Children {
//     name:String,
//  }

//  pub trait Primary_passed {
//      fn Pp(&self) -> i32 {
//          1
//      }
//  }
//  pub trait Bilingual {
//      fn bil(&self) -> i32 {
//          1
//      }
//  }

//  impl Primary_passed for Children {}
//  impl Bilingual for Children {}

//  fn main () {
//      let my_children = Children {
//          name :String::from("Yateem khana"),
//      };
//      let m = my_children.Pp();
//      let n = my_children.bil();

//   if m == 1 && n == 1 {
//       println!("Mr Asim should adopt these children");
//   }
//   else {println!("Mr Asim should'nt adopt these children");
//   }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Q5.What are the 4 qualities of closure?
//The 4 qualities of closure:
//*making a closure with no argument. 
//* capturing values from environment. 
//* passing a closure as an argument to a function.
//*a closure expecting u32 argument and returns a u32 value is passed as an argument to a function.

//////////////////////////////////////////The End///////////////////////////////////////////////////////////