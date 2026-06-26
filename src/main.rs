
fn main (){
   let mut v1=vec![1,2,3,4,5];
   
   let iter=v1.iter_mut();
   for i in iter{
    *i =*i+1;
   }
   println!("{:?}",v1);
}