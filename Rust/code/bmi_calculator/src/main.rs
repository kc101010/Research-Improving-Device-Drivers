
use std::io;

fn main() {
    //print program title
    println!("~~~~~ BMI Calculator ~~~~~");
    
    //create a new string to handle input
    let mut input = String::new();
    
    //ask for input height, read height to input string, handle error(s) if necessary
    println!("Please enter your height in Metres");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    //create a new 64bit float variable for height, cast the input string to a float and store.
    let height: f64 = input.trim().parse().unwrap();
    
    //REMEMBER TO CLEAR VARIABLES IF THEY ARE USED AGAIN!!!!!!
    input.clear();
    
    //ask for input weight, read to string, etc
    println!("Please enter your weight in Kg");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
     
    //create a new f64 for weight, cast and store string
    let weight: f64 = input.trim().parse().unwrap();
   
    //print message and call calculation function
    println!("Calculating BMI...");
    calculate_bmi(height, weight);

}

//function calculates user BMI and prints messages based on inputs
fn calculate_bmi(height: f64, weight:f64 ){
   //bmi calculation stored in new float var
   let bmi: f64 = weight / (height * height);
   
   /*if height or weight is 0, print a notification and restart the program
   /
   /BUG: Using recursion leads to an issue where the `bmi: NaN` shows up after all other messages
   /     I think this is the recursion 'rounding off' or 'finishing up' by returning to the first
   /     loop
   */ 
   if height == 0.0  || weight == 0.0 {
     println!("Your height or weight was invalid, please try again");
     main();
   }

   //print user BMI
   println!("Your BMI is {:.64}", bmi);

   //check BMI against these standards and print a related message 
   if bmi < 18.5{
        println!("You are underweight");
   }else if bmi >= 18.5 && bmi <= 24.9{
        println!("You are normal weight");
   }else if bmi >= 25.0 && bmi <= 29.9{
        println!("You are overweight");
   }else if bmi >= 30.0 && bmi <= 34.9{
        println!("You are obese");
   }else if bmi > 35.0{
        println!("You are extremely obese");
   }
}
