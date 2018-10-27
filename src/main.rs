
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

    for index in 0..lyrics.len() {
        let [nth, _] = lyrics[index];
        println!("On the {} day of Christmas, my true love gave to me", nth);

        for i in (0..index + 1).rev() {
            let [_, gift] = lyrics[i];
            println!("{}", gift);
        }

        println!("\n");
    }
}
