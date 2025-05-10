#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_must_use)]

use std::io::{stdin, stdout};
// mod models;
// use crate::models::todo::{ToDo, ToDoList};

// fn main() {
//     let mut list: ToDoList = ToDoList{
//         items: None
//     };

//     list.add(
//         "First",
//         "Description for first"
//     );
    
//     println!("{:?}",list)


// }

#[derive(Debug)]
struct ToDo{
    id: i32,
    title: String,
    desc: String
}

#[derive(Debug)]
struct ToDoList{
    items: Option<Vec<ToDo>>
}

impl ToDoList{

    fn add(&mut self, title: &str, desc: &str){
        let mut id: i32 = 1;

        match &mut self.items{
            Some(v) => {
                id = v.len() as i32 + 1;
                v.push(ToDo{
                    id: id,
                    title: title.to_string(),
                    desc: desc.to_string()
                })
            },
            None => self.items = Some(vec![ToDo{
                id: id,
                title: title.to_string(),
                desc: desc.to_string()
            }])
        }
    }

    fn display(&self, offset: usize, limit: usize){

        let in_limit: usize = limit + offset;

        match &self.items{
            Some(v) => {
                println!("Lenght of vector {:?}", v.len());

                if offset <= 2 && v.len() <= 2{
                    println!("{:?}", &v)    
                }
                else if in_limit >= v.len(){
                println!("{:?}", &v[offset..])
                }
                
                else{
                    println!("{:?}", &v[offset..in_limit])
                }
                
        },
            None => println!("No itmes")
        }
    }

}

use std::any::type_name;

fn print_type_of<T>(_: &T) -> String{
    return type_name::<T>().to_string();
}


fn main() {
    println!("
    Welcom to To Do Menu
    This application is for todo list
    You may perfrom addition, read, edit and delete operations
    ");

    loop{
        println!("
        Etner:[1,2,3,4,5]
        1> Read ToDo list
        2> Add ToDo
        3> Edit ToDo
        4> Delete
        5> Exit
        ");
        let mut user_input = String::new();
        stdin().read_line(&mut user_input);
        match &user_input[..1] {
            "1" => {
                println!("You entered {:?}", user_input);
                
            },
            "2" => {
                println!("You entered {:?}", user_input);
            },
            "3" => {
                println!("You entered {:?}", user_input);
            },  
            "4" => {
                println!("You entered {:?}", user_input);
            },
            "5" => {
                println!("You entered {:?}", user_input);
                break;
            },
            _ => continue,
        }
    }

}
// }