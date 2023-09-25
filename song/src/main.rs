fn main() {
    let text = [
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];
    let months = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "11th", "12th",
    ];
    let text2: String = "And partridge in a pear tree.\n".to_owned();
    println!("On the {} day of Christmas,", months[0]);
    println!("my true love sent to me\nA partridge in a pear tree.\n");

    for i in 1..=11 {
        println!("On the {} day of Christmas,", months[i]);
        println!("my true love sent to me");
        let mut j = i;
        while j > 0 {
            println!("{}", text[j - 1]);
            j -= 1;
        }
        println!("{text2}");
    }
}
