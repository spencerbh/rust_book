fn main() {
    let days_of_xmas = ["A partridge in a pear tree.", 
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five gold rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Tweleve drummers drumming,"];

    let days = ["first", 
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "tweleveth"];

    for day in 0..13 {
        //println!("{}", day); 
        for today in (0..day).rev() {
            if today == day-1 {
                println!("\nOn the {} day of Christmas my true love gave to me",  
            days[day-1]);
            }
            if today == 0 {
                if day != 1 {
                    println!("And")}}
            println!("{}", days_of_xmas[today])
        } 
    }
}
