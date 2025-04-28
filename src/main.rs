#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
// use std::io::{stdin, stdout, Write};
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
    title: String,
    desc: String
}

#[derive(Debug)]
struct ToDoList{
    items: Option<Vec<ToDo>>
}

impl ToDoList{

    fn add(&mut self, title: &str, desc: &str){
        match &mut self.items{
            Some(v) => v.push(ToDo{
                title: title.to_string(),
                desc: desc.to_string()
            }),
            None => self.items = Some(vec![ToDo{
                title: title.to_string(),
                desc: desc.to_string()
            }])
        }
    }

    fn display(&self, offset: usize, limit: usize){

        let in_limit: usize = limit + offset;

        match &self.items{
            Some(v) => {
                if in_limit > v.len(){
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


fn main() {

    let mut list: ToDoList = ToDoList{
        items: Option::None
    };

    list.add(
        "first",
        "Desc"
    );

    
    list.add(
        "Second",
        "Desc 2"
    );

    
    list.add(
        "Third",
        "3"
    );


    list.display(1, 4);
}
    // let v = vec![1,2,3];

    // println!("{:?}", &v[0..]);


// }