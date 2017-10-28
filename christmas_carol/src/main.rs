fn main() {
    println!("The Twelve Days of Christmas \n");

    let items = [
        "And a partridge in a pear tree!",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings!",
        "Six geese a layin'",
        "Seven swans a swimmin'",
        "Eight maids milkin'",
        "Nine ladies dancin'",
        "Ten lords a leapin'",
        "Eleven pipers pipin'",
        "Twelve drummers drummin'"
    ];

    let ordinals = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eigth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    for n in 0..11 {
        println!("On the {} day of Christmas", ordinals[n]);
        println!("My true love sent to me");
        for x in (0..n+1).rev() {
            if n == 0 && x == 0 {
                println!("A partridge in a pear tree!");
            } else {
                println!("{}", items[x]);
            }
        }
        println!("\n");
    }
}
