
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    /*
    let x = rand::random::<u8>();
println!("{}", x);

let y = rand::random::<f64>();
println!("{}", y);

if rand::random() { // generates a boolean
    println!("Better lucky than good!");
}
*/

    println!("Rate die Zahl!");
    let geheime_zahl = rand::thread_rng().gen_range(1, 101);

   println!("Die geheime Zahl ist: {}", geheime_zahl);

loop {
    println!("Bitte gib deine Vermutung ein.");

    let mut vermutung = String::new();


    io::stdin().read_line(&mut vermutung)
        .ok()
        .expect("Fehler beim Lesen der Zeile");

        let vermutung: u32 = vermutung.trim().parse()
         .ok()
         .expect("Bitte eine Zahl eintippen!");


    println!("Deine Vermutung: {}", vermutung);
    match vermutung.cmp(&geheime_zahl) {
           Ordering::Less    => println!("Zu klein!"),
           Ordering::Greater => println!("Zu groß!"),
           Ordering::Equal   => {println!("Gewonnen!");break;}
       }}
}
