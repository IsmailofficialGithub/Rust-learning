
fn main (){
   let  v1=vec![1,2,3,4,5];
    let mut v1_iter =v1.iter();

    while let Some (val)=v1_iter.next(){
        println!("{}",val);
    }

    let into_iter=v1.into_iter();
    for value in into_iter{
        println!("{}",value);
    }
   
}