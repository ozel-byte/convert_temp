use std::io;
fn main() {   
    home();
}

fn home(){
    
    option_menu();

    let option: i32 = temperature_type();
 
    let value: i32 = value_to_convert();
    
 
     if option == 1 {
         convert_temp_fahrenheit(value);
     }else if option == 2{
         convert_temp_celsius(value);
     }
}

fn option_menu(){
    println!("--Menu--");
    println!("1.-Fahrenheit");
    println!("2.-Celsius");
}

fn temperature_type() -> i32{
    println!("choose the temperature");
    let mut opcion = String::new();

    io::stdin().read_line(&mut opcion).expect("failed to read_line");

    let opcion: i32 = opcion.trim().parse().expect("failed convert numbert");

    opcion
}

fn value_to_convert() -> i32{
 
   println!("write the value to convert");
    let mut value_covert_temp = String::new();

    io::stdin().read_line( &mut value_covert_temp).expect("failed to code");

    let value_covert_temp: i32 = value_covert_temp.trim().parse().expect("failed type number");

    value_covert_temp
}

fn convert_temp_fahrenheit(value_covert_temp: i32){
    let c:f32 = (value_covert_temp as f32 - 32 as f32) / 1.8;
    println!("the value is: {} - Celsius",c);
}
fn convert_temp_celsius(value_covert_temp: i32){
    let f:f32 = (value_covert_temp as f32 * 1.8)+32 as f32;
    println!("the value is: {} - Fahrenheit",f);
}
