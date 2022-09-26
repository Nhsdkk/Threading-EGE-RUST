#[warn(non_snake_case)]

use std::thread;
use std::time::Instant;


fn _simple_numbers (max:i32) -> Vec<i32>{
    let mut res_vector: Vec<i32> = Vec::new();
    
    let mut i:i32 = 1;
    let mut is_simple:bool = true;


    while i<max+1 {

        if i == 1 {
            res_vector.push(i);
            i+=1;
            // println!("");
            continue;
        }

        for &num in res_vector.iter() {
            if i%num == 0 && num != 1 {
                is_simple = false;
                // println!("NOT SIMPLE: {}",num);
                break;
            }
        }

        if is_simple {
            res_vector.push(i);
            i+=1;
            continue;
        }else {
            is_simple = true;
            i+=1;
            continue;
        }
    }

    // for item in res_vector.iter(){
    //     println!("ITEM {}",item);
    // }

    // println!("RES_VECTOR: {}",&res_vector);

    return res_vector;
}

fn _check_number(number:&i32,dividers:Vec<i32>,&amount_of_dividers:&i32){
    let mut dividers_counter:i32 =0;

    for divider in dividers.iter() {
        if number % divider == 0 && divider != number && divider!=&1 {
            dividers_counter+=1;
        }
    }

    // println!("NUMBER {}",number);
    // println!("AMOUNT OF DIVIDERS {}",dividers_counter);

    if dividers_counter == amount_of_dividers {
        println!("Found Number {}",number);
    }
}

fn check_number_simple (number:&i32,&amount_of_dividers:&i32){
    let mut dividers_counter:i32 = 0;

    let number_clone = number.clone();

    let max = (number_clone/2)+1;

    for divider in 2..max{
        if number % divider == 0 && divider != number_clone{
            dividers_counter+=1;
        }
    }

    // println!("NUMBER {}",&number);
    // println!("AMOUNT OF DIVIDERS {}",&dividers_counter);

    if dividers_counter == amount_of_dividers {
        println!("Found Number {}",number);
    }
}

fn main() {
    let now = Instant::now();

    // let divider_vec = simple_numbers(210300/2);

    let max = 210300;
    let min =210235;

    for i in min..max{

        // let divider_vec_copy = divider_vec.clone();

        let thread = thread::spawn(move || check_number_simple(&i,&4));

        thread.join().unwrap();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}
