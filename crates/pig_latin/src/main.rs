mod pig_latin;
use pig_latin::Convertible;

#[allow(clippy::too_many_lines)]
fn test_pig_latin_conversion() {
    #[rustfmt::skip]
    let test_cases = [
        (
            "",
            ""
        ),
        (
            "    ",
            "    "
        ),
        (
            r#"  . ! ??? `` ` "" " "#,
            r#"  . ! ??? `` ` "" " "#
        ),
        (
            "a b c d e f g h i j k l m n o p q r s t u v w x y z",
            "a-hay b c d e-hay f g h i-hay j k l m n o-hay p q r s t u-hay v w x y z"
        ),
        (
            "first",
            "irst-fay"
        ),     
        (
            "apple",
            "apple-hay"
        ),
        (
            "FIRST",
            "IRST-FAY"
        ),     
        (
            "APPLE",
            "APPLE-HAY"
        ),
        (
            "First",
            "Irst-fay"
        ),     
        (
            "Apple",
            "Apple-hay"
        ),
        (
            "fIrSt",
            "IrSt-fay"
        ),     
        (
            "aPpLe",
            "aPpLe-hay"
        ),
        (
            "FiRsT",
            "IRsT-fay"
        ),     
        (
            "ApPlE",
            "ApPlE-hay"
        ),
        (
            "This is an example of Pig Latin. As you can see, it is silly, but lots of fun for children.",
            "Is-thay is-hay an-hay example-hay of-hay Ig-pay Atin-lay. As-hay ou-yay an-cay ee-say, it-hay is-hay illy-say, ut-bay ots-lay of-hay un-fay or-fay ildren-chay."
        ),
        (
            "MARK'S toy is busted. It's broken. MARK's toy can't be repaired.",
            "ARK'S-MAY oy-tay is-hay usted-bay. It's-hay oken-bray. ARK's-may oy-tay an't-cay e-bay epaired-ray."
        ),
        (
            "'Twas the night before Christmas so we prepared for Santa: cookies, milk.",
            "'As-tway e-thay ight-nay efore-bay Istmas-chray o-say e-way epared-pray or-fay Anta-say: ookies-cay, ilk-may."
        ),
        (
            "During summer, there are 38°C outside.",
            "Uring-day ummer-say, ere-thay are-hay 38°C outside-hay."
        ),
        (
            "There's 1A of current flowing through the circuit.",
            "Ere's-thay A-1AY of-hay urrent-cay owing-flay ough-thray e-thay ircuit-cay."
        ),
        (
            "That is a well-written report!",
            "At-thay is-hay a-hay ell-way-itten-wray eport-ray!"
        ),
        (
            "Voulez-vous écrire du code?",
            "Oulez-vay-ous-vay ire-écray u-day ode-cay?"
        ),
        (
            "ce\u{0308}e e\u{0308}e CE\u{0308}E \u{212b}CA \u{0041}\u{030a}CA ΎΣA ΎΣa 农历新年ca",
            "e-ce\u{0308}ay e-e\u{0308}ay E-CE\u{0308}AY A-\u{212b}CAY A-\u{0041}\u{030a}CAY A-ΎΣAY A-ύ\u{03C3}ay 农历新年a-cay"
        )
    ];

    let mut all_tests_passed = true;

    for (val_to_test, expected_result) in test_cases {
        let actual_result_from_str = val_to_test.to_pig_latin();
        let actual_result_from_string = String::from(val_to_test).to_pig_latin();

        println!("String result for");
        println!("{val_to_test}");
        println!("is");
        println!("{actual_result_from_string}");
        println!("while &str result is");
        println!("{actual_result_from_str}");
        print!("which are ");

        if actual_result_from_str == actual_result_from_string {
            println!("IDENTICAL.");
        } else {
            println!("DIFFERENT.");

            all_tests_passed = false;
            break;
        }

        print!("The results are ");

        if actual_result_from_str == expected_result {
            println!("CORRECT.");
            println!(".......................");
        } else {
            println!("INCORRECT. Expected");
            println!("{expected_result}");
            println!(".......................");

            all_tests_passed = false;
            break;
        }
    }

    if all_tests_passed {
        println!("All tests passed.");
    } else {
        println!("A test failed.");
    }
}

fn main() {
    test_pig_latin_conversion();
}
