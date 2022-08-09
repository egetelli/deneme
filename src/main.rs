use std::io;


struct Arabalar{
    Honda: u32,
    Mercedes: u32,
    Bmw: u32,
    Toyota: u32
}

fn main() {
    
    


    loop{

    println!("Sizce zam mı gelmeli indirim mi ? (z/i)");

    let mut answer = String::new();
    io::stdin().read_line(&mut answer);
    let answer = answer.trim().parse::<i32>().expect("invalid input");
    greet(&answer);
     
}

}

fn greet(answer: &i32) {
    let mut ag = Arabalar {Honda: 10000, Mercedes: 30000, Bmw: 25000, Toyota: 15000};
    println!("Honda'nın fiyatı {} $, Mercedes'nın fiyatı {} $, Bmw'nın fiyatı {} $, Toyota'nın fiyatı {} $",ag.Honda,ag.Mercedes,ag.Bmw,ag.Toyota);
    match answer {
        1 => {
            println!("YENI ARABA FIYATLARI");
            println!("Honda'nın fiyatı {} $, Mercedes'nın fiyatı {} $, Bmw'nın fiyatı {} $, Toyota'nın fiyatı {} $",ag.Honda+5000,ag.Mercedes+5000,ag.Bmw+5000,ag.Toyota+5000);
        },
        2 => {
            println!("YENI ARABA FIYATLARI");
            println!("Honda'nın fiyatı {} $, Mercedes'nın fiyatı {} $, Bmw'nın fiyatı {} $, Toyota'nın fiyatı {} $",ag.Honda-5000,ag.Mercedes-5000,ag.Bmw-5000,ag.Toyota-5000);
        }
        _ => println!("Invalid input: {}", answer),
    }
}
