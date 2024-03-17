use core::num;

fn main() {
    //8-bit integer
    let x:i8 = 10;

    println!("{}", x);

    //unsigned integer: use only if it is a +ve integer
    let y:u8 = 10;

    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}", y);

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", binary);
    println!("{}", octal);

    let byte = b'A';
    println!("{}", byte);

    let x = 2.0;
    let y: f32 = 1.0;

    let t = true;
    let f: bool = false;

    let c = 'c';

    println!("{}", c);
    println!("{}", x);
    println!("{}", y);
    println!("{}", t);
    println!("{}", f);

    let tup = (500, "hi", true);

    println!("{}", tup.0);

    let (x, y, z) = tup;

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);


    // array
    let arr = [1,2,3];

    println!("{}", arr[0]);

    let mut arr2: [i32; 3] = [4,5,6];
    print!("{}", arr2[0]);

    arr2[0] = 10;

    println!("{}", arr2[0]);

    //vectors
    let mut nums = vec![1,2,3];

    nums.push(4);
    println!("{:?}", nums);

    nums.pop();
    println!("{:?}", nums);

    let mut vec = Vec::new(); //call Vec macro constructor
    vec.push("TEST");
    vec.push("String");
    println!("{:?}", vec);

    let mut vect = Vec::<i32>::with_capacity(2);
    println!("{}", vect.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    let sv: &[i32] = &v[2..4]; //pointing to the slice. by reference

    println!("{:?}", sv);

    let name: String = String::from("Nikhil");
    let course: String = "Rust".to_string();
    let new_name = name.replace("Nikhil", "Londhe");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    // &str = "string slice" or "stir"
    let str1 = "hello"; //&str
    let str2 = str1.to_string();
    let str3 = &str2;

    println!("{}", str1);
    println!("{}", str2);
    println!("{}", str3);

    // compare string
    println!("{}", "ONE".to_lowercase() == "one");

    // string literals: where you do not want a valid utf-8 char sequence

    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);


}
 