// Iterators

            //Interating using for loop

// fn main (){
//     let nums = vec![2,2,3,4,5,35,34];
//     for num in nums {
//         println!("{}",num);
//     }
// }


            //Iterating after creating and "Iterator"

            

// fn main (){
//     let nums = vec![2,2,3,4,5,35,34];
//     let iter = nums.iter();
//     for value in iter {
//         println!("{}",value)
//     }

// }


            // Iterating using iter_mut 


// fn main (){
//    let mut v1=vec![1,2,3,4,5];
   
//    let iter=v1.iter_mut();
//    for i in iter{
//     *i =*i+1;
//    }
//    println!("{:?}",v1);
// }

        //   Iterators using next() with while loop

        
// fn main (){
//    let  v1=vec![1,2,3,4,5];
//     let mut v1_iter =v1.iter();

//     while let Some (val)=v1_iter.next(){
//         println!("{}",val);
//     }
   
// }


        //ierators using into_iter()
        
fn main (){
   let  v1=vec![1,2,3,4,5];
   

    let into_iter=v1.into_iter();
    for value in into_iter{
        println!("{}",value);
    }
   
}


// iter() [if you want immutable references to the inner variables and dont want to transfer ownership]

// iter_Mut() [if you wnat mutable references to the inner variables and dont want to transfer ownership]

// iterinto() [if you want to move the variable into the iterator and dont want to use it afterwards]