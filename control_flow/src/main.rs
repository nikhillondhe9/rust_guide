fn main() {
    // let one = 1;

    // if one > 10{
    //     println!("True");
    // } else if one == 1{
    //     println!("Equal");
    // } 
    // else{
    //     println!("False");
    // }

    //loop
    // loop{
    //     println!("Loop!");
    // }

    let mut num = 0;
    // 'counter: loop{
    //     println!("Count: {}", num);
    //     let mut decrease = 5;
    //     loop{
    //         println!("Decreasing: {}", decrease);
    //         if decrease == 4{
    //             break;
    //         }
    //         if num == 2 {
    //             break 'counter;
    //         }
    //         decrease -= 1;
    //     }
    //     num += 1;
    // }

    while num < 5{
        println!("Num: {}", num);
        num += 1;
    }

    let vec: Vec<i8> = (0..10).collect();

    for element in vec{
        println!("{}", element);
    }

    for num in (1..4).rev() {
        println!("{}", num)
    }
    println!("LIFT OFF!");
}
