fn main() {
    /*
    1. Create three variables with the names: val1, val2, and ans.
    We want to perform a simple operation of generating the modulo of val1 and val2. Set val1 to 5 and val2 to 2.
    Assign the answer to the ans variable. Before executing your code, what do you think the answer will be?
     */

    let val1 = 5;
    let val2 = 2;
    let ans = val1%val2;

    println!("{}", ans);

    /*
    2. Create a vector and put in the values "2, 4, 6, 8, 10".
    Once you have created the vector perform the following: print out the current values,
    remove the value 10, add the value 12, and then print the vector back out to confirm your results.
     */

    let mut vect: Vec<i32> = vec![2, 4, 6, 8, 10];
    
    println!("{:?}", vect);

    vect.pop();
    println!("{:?}", vect);
    vect.push(12);

    println!("{:?}", vect);

    let str1 = String::from("Hello");
    let ans = concat2_string(str1);
    println!("{}",ans);

    control_flow(26);

}

/*
3. Create a function called "concat_string".
Create a string variable and assign the value "Hello" to it.
The function is going to take one argument that is of type string and is going to return a String.
Inside this function, concatenate the string " World".
Print out the results in main() to confirm your results.
*/
fn concat2_string(val: String) -> String{
    val + " World"
}

/*
4. Create a function called control_flow. This is going to take one argument that is an integer.
Based on this integer, print out the following: "The value is one",
"The value is greater than 50", "The value is less than 25", or "The value is greater than 25 but less than 50".
*/
fn control_flow(num: i32){
    if num > 50{
        println!("The value is greater than 50");
    }else if num < 25{
        println!("The Value is less than 25");
    } else if num > 25 && num < 50{
        println!("The value is greater than 25 but less than 50");
    }else{
        println!("The Value is One");
    }
}