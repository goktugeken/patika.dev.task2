

fn main() {
    let mut a=10.0;
    let mut b=20.0;
    let number=-64.0;
 
    let sum_result=find_sum(a, b);
    println!("the 10+20 result {}",sum_result);
    let ext_result=find_ext(a, b);
    println!("the 10-20 result {}",ext_result);
    let multip_result=find_multip(a, b);
    println!("the 10*20 result {}",multip_result);
    let c=0.0;
 
    let divide_result=find_divide(a, c);
 
    match divide_result{
     Ok(value) =>println!("{} divided by {} is: {}", a, b, value),
     Err(error_message) =>print!("Error {} ",error_message),
    }
 
    let square_root_Result=find_square_root(number);
    match square_root_Result{
     Some(value) => print!("the square root of {} is {}",number,value),
     None => println!("The square root of {} is not a real number.", number),
       }
    
    let square_result=find_squarring(a);
    println!("the square of {} is {}",a,square_result );
 
 
 }
 
 fn find_squarring(number:f64) -> f64 {
     number*number
 }
 fn find_square_root (number:f64) -> Option<f64>{
     if number >= 0.0 {
         Some(number.sqrt())
     }
     else {
         None 
     }
    
 
 }
 fn find_sum(a:f64 , b:f64) -> f64{
      a+b
         
     }
 
 fn find_multip(a:f64,b:f64) ->f64 {
     a*b
 }
 fn find_divide(a:f64,b:f64)-> Result<f64,String>{
     if b ==0.0 {
         Err("division by zero is not allowed.".to_string())
     }
     else{
         Ok(a/b)    }
     
 }
 fn find_ext(a:f64,b:f64) ->f64 {
     a-b
 }
