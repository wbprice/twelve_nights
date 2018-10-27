
fn main() {
    
    let lyrics = [
        ["first", "A partridge in a pear tree"],
        ["second", "Two turtle doves"],
        ["third", "Three french hens"],
        ["fourth", "Four calling birds"],
        ["fifth", "Five golden rings"],
        ["sixth", "Six geese-a-laying"],
        ["seventh", "Seven swans-a-swimming"],
        ["eighth", "Maids-a-milking"],
        ["ninth", "Nine ladies dancing"],
        ["tenth", "Ten lords-a-leaping"],
        ["eleventh", "Eleven pipers piping"],
        ["twelfth", "Twelve drummers drumming"],
    ];

    let mut index = 0;
    while index < 12 {
        let [nth, _] = lyrics[index];
        let mut i = index;
        println!("On the {} day of Christmas, my true love gave to me", nth);
        while i > 0 {
            let [_, gift] = lyrics[i];
            println!("{}!", gift);
            i = i - 1;
        }
        index = index + 1;
        println!("\n");
    }
}
