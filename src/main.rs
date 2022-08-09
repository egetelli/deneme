use std::io;


struct Arabalar{
    Honda: u32,
    Mercedes: u32,
    Bmw: u32,
    Toyota: u32
}

fn main() {
    
    let mut ag = Arabalar {Honda: 10000, Mercedes: 30000, Bmw: 25000, Toyota: 15000};

    println!("Honda'nın fiyatı {} $, Mercedes'nın fiyatı {} $, Bmw'nın fiyatı {} $, Toyota'nın fiyatı {} $",ag.Honda,ag.Mercedes,ag.Bmw,ag.Toyota);


    loop{

    println!("Sizce zam mı gelmeli indirim mi ? (z/i)");

    let mut answer = String::new();
    io::stdin()
    .read_line(&mut answer)
    .expect("Failed to read line");


    let zamnveindirim = match answer.as_str() {
        "Z" | "z" => ag.Honda + 5000,ag.Mercedes + 5000,ag.Bmw + 5000,ag.Toyota + 5000,
        "I" | "i" => ag.Honda -5000,ag.Mercedes -5000,ag.Bmw -5000,ag.Toyota -5000,
        other => {
          println!("Invalid input! Expected 'Y' or 'N' but got '{}'", other);
          return;
        }

        println!(zamnveindirim);


    if answer == {"z"}{
        ag.Honda += 5000;
        ag.Mercedes += 5000;
        ag.Bmw += 5000;
        ag.Toyota += 5000;
        println!("Arabaların Yeni Fiyatı: Honda-> {}, Mercedes-> {}, Bmw-> {}, Toyota-> {},",ag.Honda + 5000,ag.Mercedes + 5000,ag.Bmw + 5000,ag.Toyota + 5000);

    }
    else if answer == {"i"}{
        ag.Honda -= 5000;
        ag.Mercedes -= 5000;
        ag.Bmw -= 5000;
        ag.Toyota -= 5000;
        println!("Arabaların Yeni Fiyatı: Honda-> {}, Mercedes-> {}, Bmw-> {}, Toyota-> {},",ag.Honda -5000,ag.Mercedes -5000,ag.Bmw -5000,ag.Toyota -5000);

    }
}

}
}
