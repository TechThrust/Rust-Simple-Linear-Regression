use std::process;
use std::io;

fn main() {
    println!("Monthly wage in $");
    let x: [f64; 8] = [18.0, 25.0, 32.0, 27.0, 34.0, 45.0, 37.0, 63.0];
    let y: [f64; 8] = [1000.0, 2500.0, 3100.0, 3400.0, 3900.0, 4300.0, 5500.0, 7300.0]; 
    if x.len() != y.len() {
        println!("X array length ({}) must be same as Y array length ({})", x.len(), y.len());
        process::exit(1);
    }
    //x-mean
    let mut x_iter = 0;
    let mut xtotal: f64 = 0.0;
    while x_iter != x.len() {
        xtotal = xtotal + x[x_iter];
        x_iter = x_iter+1;
    }
    let x_len = x.len().to_string().trim().parse::<f64>().unwrap();
    let xmean: f64 = xtotal/x_len;
    println!("Mean of X = {}", xmean);

    //y mean
    let mut y_iter = 0;
    let mut ytotal: f64 = 0.0;
    while y_iter != y.len() {
        ytotal = ytotal + y[y_iter];
        y_iter = y_iter+1;
    }
    let y_len = y.len().to_string().trim().parse::<f64>().unwrap();
    let ymean: f64 = ytotal/y_len;
    println!("Mean of Y = {}", ymean);

    //x*y, x*x (x squared), y*y (y squared)
    x_iter = 0;
    let mut xy: f64 = 0.0;
    let mut xx: f64 = 0.0;
    let mut yy: f64 = 0.0;
    while x_iter != x.len() {
        xy = xy+(x[x_iter]*y[x_iter]);
        xx = xx+(x[x_iter]*x[x_iter]);
        yy = yy+(y[x_iter]*y[x_iter]);
        x_iter = x_iter+1;
    }
    println!("Sigma(xy) = {}", xy);
    println!("Sigma(xx) = {}", xx);
    println!("Sigma(yy) = {}", yy);

    //Work out a in y=bx+a
    let a: f64 = ((ytotal*xx)-(xtotal*xy))/((x_len*xx)-(xtotal*xtotal));
    println!("a (y-intercept) = {}", a);

    //Work out b in y=bx+a
    let b: f64 = ((x_len*xy)-(xtotal*ytotal))/((x_len*xx)-(xtotal*xtotal));
    println!("b (slope) = {}", b);

    //Ask for input
    println!("Do you want to workout y or x?");
    let mut x_or_y = String::new();
    io::stdin().read_line(&mut x_or_y).expect("There was an error");
    let x_or_y_trim = x_or_y.trim();
    if x_or_y_trim == "y" {
        let mut what_x = String::new();
        println!("What is X?");
        io::stdin().read_line(&mut what_x).expect("There was an error");
        let what_x_f64: f64 = what_x.trim().parse::<f64>().unwrap();
        let y_equation = (b*what_x_f64)+a;
        println!("Estimated Y = {}", y_equation);
    }

    if x_or_y_trim == "x" {
        let mut what_y = String::new();
        println!("What is Y");
        io::stdin().read_line(&mut what_y).expect("There was an error");
        let what_y_f64: f64 = what_y.trim().parse::<f64>().unwrap();
        let x_equation = (what_y_f64-a)/b;
        println!("Estimated X = {}", x_equation);
    }
}  
