const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

macro_rules! day_lyric_format {
    () => {
        "On the {} day of Christmas my true love sent to me"
    };
}

macro_rules! first_gift_lyric_in_the_first_day {
    () => {
        "A partridge in a pear tree."
    };
}

const GIFT_LYRICS: [&str; 12] = [
    "And a partridge in a pear tree.",
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
    "Twelve drummers drumming,",
];

fn main() {
    println!(day_lyric_format!(), DAYS[0]);
    println!(first_gift_lyric_in_the_first_day!());

    for (i, day) in DAYS.iter().enumerate().skip(1) {
        println!();
        println!(day_lyric_format!(), day);

        for gift_lyric in GIFT_LYRICS.iter().take(i + 1).rev() {
            println!("{gift_lyric}");
        }
    }
}
