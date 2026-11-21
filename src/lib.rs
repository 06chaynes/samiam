mod words;
use std::f32::consts::{PI, TAU};

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

// Letter-to-sound rules after NRL Report 7948 as (left context, letters, right context, phonemes),
// tried in order; a letter no rule covers says itself. In a context a space is a word edge, `#` one
// or more vowels, `:` any consonants, `^` one consonant, `.` a voiced consonant, `+` E, I or Y, `@`
// a consonant that makes U say oo, `&` a sibilant and `%` a suffix such as -ED or -ING.
static RULES: [(&str, &str, &str, &str); 320] = [
    ("", "A", " ", "AX"),
    (" ", "AR", "O", "AX R"),
    ("", "AR", "#", "EH R"),
    ("", "A", "WA", "AX"),
    ("", "AW", "", "AO"),
    (" :", "ANY", "", "EH N IY"),
    ("", "A", "^+#", "EY"),
    ("^", "AS", "#", "EY S"),
    ("#:", "ALLY", "", "AX L IY"),
    (" ", "AL", "#", "AX L"),
    ("", "AGAIN", "", "AX G EH N"),
    ("#:", "AG", "E", "IH JH"),
    ("", "A", "^+:#", "AE"),
    (" :", "A", "^+ ", "EY"),
    ("", "A", "^%", "EY"),
    (" ", "ARR", "", "AX R"),
    ("", "ARR", "", "AE R"),
    (" :", "AR", " ", "AA R"),
    ("", "AR", " ", "ER"),
    ("", "AR", "", "AA R"),
    ("", "AIR", "", "EH R"),
    ("", "AI", "", "EY"),
    ("", "AY", "", "EY"),
    ("", "AU", "", "AO"),
    ("#:", "AL", " ", "AX L"),
    ("#:", "ALS", " ", "AX L Z"),
    ("", "ALK", "", "AO K"),
    ("", "AL", "^", "AO L"),
    (" :", "ABLE", "", "EY B AX L"),
    ("", "ABLE", "", "AX B AX L"),
    ("", "ANG", "+", "EY N JH"),
    ("", "A", "", "AE"),
    (" ", "BE", "^#", "B IX"),
    ("", "BEING", "", "B IY IH NG"),
    (" ", "BOTH", " ", "B OW TH"),
    (" ", "BUS", "#", "B IH Z"),
    ("", "BUIL", "", "B IH L"),
    (" ", "CH", "^", "K"),
    ("^E", "CH", "", "K"),
    ("", "CH", "", "CH"),
    (" S", "CI", "#", "S AY"),
    ("", "CI", "A", "SH"),
    ("", "CI", "O", "SH"),
    ("", "CI", "EN", "SH"),
    ("", "C", "+", "S"),
    ("", "CK", "", "K"),
    (" ", "COM", "PU", "K AX M"),
    ("", "COM", "%", "K AH M"),
    ("", "C", "", "K"),
    ("", "DOLLAR", "", "D AA L ER"),
    ("#:", "DED", " ", "D IH D"),
    (".E", "D", " ", "D"),
    ("#:^E", "D", " ", "T"),
    (" ", "DE", "^#", "D IX"),
    (" ", "DOES", "", "D AH Z"),
    (" ", "DOING", "", "D UW IH NG"),
    (" ", "DOW", "", "D AW"),
    ("", "DU", "A", "JH UW"),
    ("#:", "E", " ", ""),
    (" :", "E", " ", "IY"),
    ("#", "ED", " ", "D"),
    ("#:", "E", "D ", ""),
    ("", "EV", "ER", "EH V"),
    ("", "E", "^%", "IY"),
    ("", "ERI", "#", "IY R IY"),
    ("", "ERI", "", "EH R IH"),
    ("#:", "ER", "#", "ER"),
    ("", "ERR", "", "EH R"),
    ("", "ER", "#", "EH R"),
    ("", "ER", "", "ER"),
    (" ", "EVEN", "", "IY V EH N"),
    ("#:", "E", "W", ""),
    ("@", "EW", "", "UW"),
    ("", "EW", "", "Y UW"),
    ("", "E", "O", "IY"),
    ("#:&", "ES", " ", "IH Z"),
    ("#:", "E", "S ", ""),
    ("#:", "ELY", " ", "L IY"),
    ("#:", "EMENT", "", "M EH N T"),
    ("", "EFUL", "", "F UH L"),
    ("", "EE", "", "IY"),
    ("", "EARN", "", "ER N"),
    (" ", "EAR", "^", "ER"),
    ("", "EAD", "", "EH D"),
    ("#:", "EA", " ", "IY AX"),
    ("", "EA", "SU", "EH"),
    ("", "EA", "", "IY"),
    ("", "EIGH", "", "EY"),
    ("", "EI", "", "IY"),
    (" ", "EYE", "", "AY"),
    ("", "EY", "", "IY"),
    ("", "EU", "", "Y UW"),
    ("", "E", "", "EH"),
    ("", "FUL", "", "F UH L"),
    ("", "GIV", "", "G IH V"),
    (" ", "G", "I^", "G"),
    ("", "GE", "T", "G EH"),
    ("SU", "GGES", "", "G JH EH S"),
    ("", "GG", "", "G"),
    (" B#", "G", "", "G"),
    ("", "G", "+", "JH"),
    ("", "GREAT", "", "G R EY T"),
    ("#", "GH", "", ""),
    ("", "HUNDRED", "", "HH AH N D R AX D"),
    (" ", "HAV", "", "HH AE V"),
    (" ", "HERE", "", "HH IY R"),
    (" ", "HOUR", "", "AW ER"),
    ("", "HOW", "", "HH AW"),
    ("", "H", "#", "HH"),
    ("", "H", "", ""),
    ("", "IETH", "", "IY AX TH"),
    (" ", "IN", "", "IH N"),
    (" ", "I", " ", "AY"),
    ("", "IN", "D", "AY N"),
    ("", "IER", "", "IY ER"),
    ("#:R", "IED", "", "IY D"),
    ("", "IED", " ", "AY D"),
    ("", "IEN", "", "IY EH N"),
    ("", "IE", "T", "AY EH"),
    (" :", "I", "%", "AY"),
    ("", "I", "%", "IY"),
    ("", "IE", "", "IY"),
    ("", "I", "^+:#", "IH"),
    ("", "IR", "#", "AY R"),
    ("", "IZ", "%", "AY Z"),
    ("", "IS", "%", "AY Z"),
    ("", "I", "D%", "AY"),
    ("+^", "I", "^+", "IH"),
    ("", "I", "T%", "AY"),
    ("#:^", "I", "^+", "IH"),
    ("", "I", "^I", "IH"),
    ("", "I", "^+", "AY"),
    ("", "IR", "", "ER"),
    ("", "IGH", "", "AY"),
    ("", "ILD", "", "AY L D"),
    ("", "IGN", " ", "AY N"),
    ("", "IGN", "^", "AY N"),
    ("", "IGN", "%", "AY N"),
    ("", "IQUE", "", "IY K"),
    (" :", "I", "#", "AY"),
    ("", "I", "#", "IY"),
    ("", "I", "", "IH"),
    ("", "J", "", "JH"),
    (" ", "K", "N", ""),
    ("", "LO", "C#", "L OW"),
    ("L", "L", "", ""),
    ("#:^", "L", "%", "AX L"),
    ("", "LEAD", "", "L IY D"),
    (" ", "MINUS", "", "M AY N AX S"),
    ("", "MICRO", "", "M AY K R OW"),
    ("", "MOV", "", "M UW V"),
    (" ", "NINE", "", "N AY N"),
    (" ", "NINTH", "", "N AY N TH"),
    ("^", "N'T", " ", "AX N T"),
    ("E", "NG", "+", "N JH"),
    ("", "NG", "R", "NG G"),
    ("", "NG", "#", "NG G"),
    ("", "NGL", "%", "NG G AX L"),
    ("", "NG", "", "NG"),
    ("", "NK", "", "NG K"),
    (" ", "NOW", " ", "N AW"),
    ("", "OF", " ", "AX V"),
    ("", "OROUGH", "", "ER OW"),
    ("#:", "OR", " ", "ER"),
    ("#:", "ORS", " ", "ER Z"),
    ("", "OR", "", "AO R"),
    (" ", "ONE", "", "W AH N"),
    ("", "OW", "", "OW"),
    (" ", "OVER", "", "OW V ER"),
    ("", "OV", "", "AH V"),
    ("", "O", "^%", "OW"),
    ("", "O", "^EN", "OW"),
    ("", "O", "^I#", "OW"),
    ("", "OL", "D", "OW L"),
    (" D", "OLL", "", "AA L"),
    (" R", "OLL", "", "OW L"),
    ("", "OLL", " ", "OW L"),
    ("", "OLL", "S ", "OW L"),
    ("", "OLL", "ED ", "OW L"),
    ("", "OLL", "ING ", "OW L"),
    ("", "OUGHT", "", "AO T"),
    ("", "OUGH", "", "AH F"),
    (" ", "OU", "", "AW"),
    ("H", "OU", "S#", "AW"),
    ("", "OUS", "", "AX S"),
    ("", "OUR", "", "AO R"),
    ("", "OULD", "", "UH D"),
    ("^", "OU", "^L", "AH"),
    ("", "OUP", "", "UW P"),
    ("", "OU", "I", "UW"),
    ("", "OU", "", "AW"),
    ("", "OY", "", "OY"),
    ("", "OING", "", "OW IH NG"),
    ("", "OI", "", "OY"),
    ("", "OOR", "", "AO R"),
    ("", "OOK", "", "UH K"),
    ("", "OOD", "", "UH D"),
    ("", "OO", "", "UW"),
    ("", "O", "E", "OW"),
    ("", "O", " ", "OW"),
    ("", "O", "S ", "OW"),
    ("", "OA", "", "OW"),
    (" ", "ONLY", "", "OW N L IY"),
    (" ", "ONCE", "", "W AH N S"),
    ("", "ON'T", "", "OW N T"),
    ("C", "O", "N", "AA"),
    ("", "O", "NG", "AO"),
    (" :^", "O", "N", "AH"),
    ("I", "ON", "", "AX N"),
    ("#:", "ON", " ", "AX N"),
    ("#^", "ON", "", "AX N"),
    ("", "O", "ST ", "OW"),
    ("", "OF", "^", "AO F"),
    ("", "OTHER", "", "AH DH ER"),
    ("", "OSS", " ", "AO S"),
    ("#:^", "OM", "", "AH M"),
    ("", "O", "", "AA"),
    ("", "PH", "", "F"),
    ("", "PEOP", "", "P IY P"),
    ("", "POW", "", "P AW"),
    ("", "PUT", " ", "P UH T"),
    ("", "QUAR", "", "K W AO R"),
    ("", "QU", "", "K W"),
    ("", "Q", "", "K"),
    (" ", "RE", "^#", "R IY"),
    ("", "SH", "", "SH"),
    ("#", "SION", "", "ZH AX N"),
    ("", "SOME", "", "S AH M"),
    ("", "SOFT", "", "S AO F T"),
    (" ", "SE", "LE", "S AX"),
    ("#", "SUR", "#", "ZH ER"),
    ("", "SUR", "#", "SH ER"),
    ("#", "SU", "#", "ZH UW"),
    ("#", "SSU", "#", "SH UW"),
    ("#", "SED", " ", "Z D"),
    ("#", "S", "#", "Z"),
    ("", "SAID", "", "S EH D"),
    ("^", "SION", "", "SH AX N"),
    ("", "S", "S", ""),
    (".", "S", " ", "Z"),
    ("#:.E", "S", " ", "Z"),
    ("U", "S", " ", "S"),
    ("#:^##", "S", " ", "Z"),
    ("#:^#", "S", " ", "S"),
    (" :#", "S", " ", "Z"),
    (" ", "SCH", "", "S K"),
    ("", "S", "C+", ""),
    ("#", "SM", "", "Z M"),
    ("#", "SN", "'", "Z AX N"),
    ("", "TO", " ", "T UW"),
    ("", "THAT", " ", "DH AE T"),
    (" ", "THIS", " ", "DH IH S"),
    (" ", "THEY", "", "DH EY"),
    (" ", "THERE", "", "DH EH R"),
    ("", "THER", "", "DH ER"),
    ("", "THEIR", "", "DH EH R"),
    (" ", "THAN", " ", "DH AE N"),
    ("", "THESE", " ", "DH IY Z"),
    (" ", "THEN", "", "DH EH N"),
    ("", "THROUGH", "", "TH R UW"),
    ("", "THOSE", "", "DH OW Z"),
    ("", "THOUGH", " ", "DH OW"),
    (" ", "THUS", "", "DH AH S"),
    ("", "TH", "", "TH"),
    ("#:", "TED", " ", "T IH D"),
    ("S", "TI", "#N", "CH"),
    ("", "TI", "O", "SH"),
    ("", "TI", "A", "SH"),
    ("", "TIEN", "", "SH AX N"),
    ("", "TUR", "#", "CH ER"),
    ("", "TU", "A", "CH UW"),
    (" ", "TWO", "", "T UW"),
    (" ", "UN", "I", "Y UW N"),
    (" ", "UN", "", "AH N"),
    (" ", "UPON", "", "AX P AO N"),
    ("@", "UR", "#", "UH R"),
    ("", "UR", "#", "Y UH R"),
    ("", "UR", "", "ER"),
    ("", "U", "^ ", "AH"),
    ("", "U", "^^", "AH"),
    ("", "UY", "", "AY"),
    (" G", "U", "#", ""),
    ("G", "U", "%", ""),
    ("G", "U", "#", "W"),
    ("#N", "U", "", "Y UW"),
    ("@", "U", "", "UW"),
    ("", "U", "", "Y UW"),
    ("", "VIEW", "", "V Y UW"),
    (" ", "WERE", "", "W ER"),
    ("", "WA", "S", "W AA"),
    ("", "WA", "T", "W AA"),
    ("", "WHERE", "", "WH EH R"),
    ("", "WHAT", "", "WH AA T"),
    ("", "WHOL", "", "HH OW L"),
    ("", "WHO", "", "HH UW"),
    ("", "WH", "", "WH"),
    ("", "WAR", "", "W AO R"),
    ("", "WOR", "^", "W ER"),
    ("", "WR", "", "R"),
    ("", "X", "", "K S"),
    ("", "YOUNG", "", "Y AH NG"),
    (" ", "YOU", "", "Y UW"),
    (" ", "YES", "", "Y EH S"),
    (" ", "Y", "", "Y"),
    ("#:F", "Y", " ", "AY"),
    ("#:^", "Y", " ", "IY"),
    ("#:^", "Y", "I", "IY"),
    (" :", "Y", " ", "AY"),
    (" :", "Y", "#", "AY"),
    (" :", "Y", "^+:#", "IH"),
    (" :", "Y", "^#", "AY"),
    ("", "Y", "", "IH"),
    (".", "'S", " ", "Z"),
    ("#:.E", "'S", " ", "Z"),
    ("#:^E", "'S", " ", "S"),
    ("#", "'S", " ", "Z"),
    ("^", "'VE", " ", "AX V"),
    ("", "'VE", " ", "V"),
    ("", "'RE", " ", "R"),
    ("", "'", "", ""),
];
// Per sound: milliseconds, three formants in Hz, then voicing, aspiration and frication levels
// and the frication centre in Hz.
const SOUNDS: [(&str, [f32; 8]); 36] = [
    ("IY", [110.0, 270.0, 2290.0, 3010.0, 1.0, 0.0, 0.0, 0.0]),
    ("IH", [80.0, 390.0, 1990.0, 2550.0, 1.0, 0.0, 0.0, 0.0]),
    ("IX", [50.0, 400.0, 1850.0, 2500.0, 1.0, 0.0, 0.0, 0.0]),
    ("EH", [90.0, 530.0, 1840.0, 2480.0, 1.0, 0.0, 0.0, 0.0]),
    ("AE", [120.0, 660.0, 1720.0, 2410.0, 1.0, 0.0, 0.0, 0.0]),
    ("AA", [120.0, 730.0, 1090.0, 2440.0, 1.0, 0.0, 0.0, 0.0]),
    ("AO", [120.0, 570.0, 840.0, 2410.0, 1.0, 0.0, 0.0, 0.0]),
    ("OH", [120.0, 540.0, 1100.0, 2300.0, 1.0, 0.0, 0.0, 0.0]),
    ("UH", [80.0, 440.0, 1020.0, 2240.0, 1.0, 0.0, 0.0, 0.0]),
    ("UW", [110.0, 300.0, 870.0, 2240.0, 1.0, 0.0, 0.0, 0.0]),
    ("ER", [110.0, 490.0, 1350.0, 1690.0, 1.0, 0.0, 0.0, 0.0]),
    ("AX", [60.0, 500.0, 1400.0, 2300.0, 1.0, 0.0, 0.0, 0.0]),
    ("AH", [80.0, 640.0, 1190.0, 2390.0, 1.0, 0.0, 0.0, 0.0]),
    ("L", [60.0, 330.0, 1050.0, 2880.0, 0.8, 0.0, 0.0, 0.0]),
    ("R", [60.0, 310.0, 1060.0, 1380.0, 0.8, 0.0, 0.0, 0.0]),
    ("W", [50.0, 290.0, 610.0, 2150.0, 0.8, 0.0, 0.0, 0.0]),
    ("Y", [50.0, 260.0, 2070.0, 3020.0, 0.8, 0.0, 0.0, 0.0]),
    ("M", [70.0, 280.0, 900.0, 2200.0, 0.6, 0.0, 0.0, NASAL]),
    ("N", [70.0, 280.0, 1700.0, 2600.0, 0.6, 0.0, 0.0, NASAL]),
    ("NG", [70.0, 280.0, 2300.0, 2750.0, 0.6, 0.0, 0.0, NASAL]),
    ("HH", [80.0, 500.0, 1500.0, 2500.0, 0.0, 0.5, 0.0, 0.0]),
    ("F", [90.0, 340.0, 1100.0, 2080.0, 0.0, 0.0, 0.1, 7500.0]),
    ("V", [60.0, 220.0, 1100.0, 2080.0, 0.5, 0.0, 0.07, 7500.0]),
    ("TH", [90.0, 320.0, 1290.0, 2540.0, 0.0, 0.0, 0.14, 5000.0]),
    ("DH", [50.0, 270.0, 1290.0, 2540.0, 0.5, 0.0, 0.1, 5000.0]),
    ("S", [100.0, 320.0, 1390.0, 2530.0, 0.0, 0.0, 0.39, 3900.0]),
    ("Z", [70.0, 240.0, 1390.0, 2530.0, 0.5, 0.0, 0.17, 3900.0]),
    ("SH", [100.0, 300.0, 1840.0, 2750.0, 0.0, 0.0, 0.98, 2700.0]),
    ("ZH", [70.0, 300.0, 1840.0, 2750.0, 0.5, 0.0, 0.63, 2700.0]),
    ("JZ", [70.0, 300.0, 1840.0, 2750.0, 0.3, 0.0, 1.2, 2700.0]),
    ("PC", [60.0, 200.0, 1100.0, 2150.0, 0.0, 0.0, 0.0, 0.0]),
    ("BC", [70.0, 200.0, 1100.0, 2150.0, 0.3, 0.0, 0.0, 0.0]),
    ("TC", [50.0, 200.0, 1600.0, 2600.0, 0.0, 0.0, 0.0, 0.0]),
    ("DC", [60.0, 200.0, 1600.0, 2600.0, 0.3, 0.0, 0.0, 0.0]),
    ("KC", [60.0, 200.0, 1990.0, 2850.0, 0.0, 0.0, 0.0, 0.0]),
    ("GC", [70.0, 200.0, 1990.0, 2850.0, 0.3, 0.0, 0.0, 0.0]),
];
const PARTS: [(&str, &str); 8] = [
    ("EY", "EH IH"),
    ("OW", "OH UH"),
    ("AY", "AA IH"),
    ("AW", "AA UH"),
    ("OY", "AO IH"),
    ("CH", "TC SH"),
    ("JH", "DC JZ"),
    ("WH", "HH W"),
];
// Each stop's closure, the frication centre of its release, and whether it is voiced.
const STOPS: [(&str, &str, f32, bool); 6] = [
    ("P", "PC", 1500.0, false),
    ("B", "BC", 1500.0, true),
    ("T", "TC", 4000.0, false),
    ("D", "DC", 4000.0, true),
    ("K", "KC", VELAR, false),
    ("G", "GC", VELAR, true),
];
// The frication centre of a k or g release.
const VELAR: f32 = 2000.0;
// Frication centres that mark a sound rather than select a noise: a diphthong's second half, a
// word boundary and a nasal. The formants move slowly into the first two.
const GLIDE: f32 = -1.0;
const BOUNDARY: f32 = -2.0;
const NASAL: f32 = -3.0;
// Each noise by the frication centre that selects it: its full level there, its loudness in dB
// below a vowel, and its band-passes as (centre Hz, bandwidth Hz, gain), where a 0 Hz centre
// follows the sound's second formant. The first four are fricatives, the rest stop releases.
type Shape = (f32, f32, f32, &'static [(f32, f32, f32)]);
const SHAPES: [Shape; 7] = [
    (7500.0, 0.1, -39.0, &[(5500.0, 6000.0, 1.0)]),
    (5000.0, 0.14, -41.0, &[(5800.0, 2500.0, 1.0)]),
    (
        3900.0,
        0.39,
        -27.0,
        &[(5000.0, 2200.0, 1.0), (7000.0, 3000.0, 0.7)],
    ),
    (
        2700.0,
        0.98,
        -27.0,
        &[(2800.0, 900.0, 1.0), (5200.0, 3000.0, 0.6)],
    ),
    (1500.0, 0.21, -16.0, &[(700.0, 1400.0, 1.0)]),
    (4000.0, 0.42, -15.0, &[(6500.0, 6000.0, 1.0)]),
    (VELAR, 0.42, -14.0, &[(0.0, 700.0, 1.0)]),
];
// Aspiration's loudness in dB below a vowel.
const ASPIRATION: f32 = -15.0;
const VOWELS: [char; 5] = ['A', 'E', 'I', 'O', 'U'];
// Consonants a vowel is drawn out before.
const VOICED: [&str; 8] = ["B", "D", "G", "V", "DH", "Z", "ZH", "JH"];
const PAUSES: [char; 4] = [',', '.', '!', '?'];
const ONES: [&str; 20] = [
    "ZERO", "ONE", "TWO", "THREE", "FOUR", "FIVE", "SIX", "SEVEN", "EIGHT", "NINE", "TEN",
    "ELEVEN", "TWELVE", "THIRTEEN", "FOURTEEN", "FIFTEEN", "SIXTEEN", "SEVENTEEN", "EIGHTEEN",
    "NINETEEN",
];
const TENS: [&str; 10] = [
    "", "", "TWENTY", "THIRTY", "FORTY", "FIFTY", "SIXTY", "SEVENTY", "EIGHTY", "NINETY",
];
const SCALES: [(u64, &str); 5] = [
    (1_000_000_000_000, "TRILLION"),
    (1_000_000_000, "BILLION"),
    (1_000_000, "MILLION"),
    (1_000, "THOUSAND"),
    (100, "HUNDRED"),
];
const ORDINALS: [(&str, &str); 8] = [
    ("ONE", "FIRST"),
    ("TWO", "SECOND"),
    ("THREE", "THIRD"),
    ("FIVE", "FIFTH"),
    ("EIGHT", "EIGHTH"),
    ("TWELVE", "TWELFTH"),
    ("Y", "IETH"),
    ("", "TH"),
];
const LETTERS: [&str; 26] = [
    "EY1", "B IY1", "S IY1", "D IY1", "IY1", "EH1 F", "JH IY1", "EY1 CH", "AY1", "JH EY1", "K EY1",
    "EH1 L", "EH1 M", "EH1 N", "OW1", "P IY1", "K Y UW1", "AA1 R", "EH1 S", "T IY1", "Y UW1",
    "V IY1", "D AH1 B AX L Y UW", "EH1 K S", "W AY1", "Z IY1",
];
const ABBREVIATIONS: [(&str, &str); 19] = [
    ("Mr.", "MISTER"),
    ("Mrs.", "MISSUS"),
    ("Ms.", "MIZ"),
    ("Dr.", "DOCTOR"),
    ("St.", "STREET"),
    ("Jr.", "JUNIOR"),
    ("Sr.", "SENIOR"),
    ("vs.", "VERSUS"),
    ("vs", "VERSUS"),
    ("etc.", "ET CETERA"),
    ("e.g.", "FOR EXAMPLE"),
    ("i.e.", "THAT IS"),
    ("km", "KILOMETERS"),
    ("kg", "KILOGRAMS"),
    ("cm", "CENTIMETERS"),
    ("lb", "POUNDS"),
    ("lbs", "POUNDS"),
    ("oz", "OUNCES"),
    ("ft", "FEET"),
];
// Endings that put the stress a set number of syllables before the word's end, longest first.
const PULLS: [(&str, usize); 51] = [
    ("GRAPHY", 2),
    ("IONAL", 2),
    ("CIENT", 1),
    ("TIENT", 1),
    ("IENCE", 2),
    ("ITIVE", 2),
    ("ATIVE", 2),
    ("ULATE", 2),
    ("ITION", 1),
    ("ITUDE", 2),
    ("ICITY", 2),
    ("TION", 1),
    ("SION", 1),
    ("CIAN", 1),
    ("ICAL", 2),
    ("CIAL", 1),
    ("TIAL", 1),
    ("IOUS", 1),
    ("EOUS", 1),
    ("UOUS", 2),
    ("IBLE", 2),
    ("ULAR", 2),
    ("LOGY", 2),
    ("IENT", 2),
    ("ENCE", 2),
    ("ANCE", 2),
    ("ENCY", 2),
    ("ANCY", 2),
    ("ICS", 1),
    ("ITY", 2),
    ("IFY", 2),
    ("UAL", 2),
    ("IUM", 2),
    ("ION", 1),
    ("ATE", 2),
    ("IZE", 2),
    ("ISE", 2),
    ("ARY", 2),
    ("ORY", 2),
    ("OUS", 2),
    ("ENT", 2),
    ("ANT", 2),
    ("IVE", 1),
    ("URE", 1),
    ("AGE", 1),
    ("ISH", 1),
    ("IC", 1),
    ("IA", 2),
    ("AL", 2),
    ("ID", 1),
    ("IT", 1),
];
// Endings that take the stress themselves.
const LAST: [&str; 14] = [
    "EEN", "SELVES", "ESQUE", "ETTE", "IQUE", "AIRE", "SELF", "EER", "OON", "ESE", "AIN", "ECT",
    "ICT", "EE",
];
// Endings that leave the stress where the shorter word has it, with the syllables they add,
// longest first.
const ADDED: [(&str, usize); 19] = [
    ("MENTS", 1),
    ("INGS", 1),
    ("MENT", 1),
    ("NESS", 1),
    ("LESS", 1),
    ("ABLE", 2),
    ("ING", 1),
    ("ERS", 1),
    ("FUL", 1),
    ("IES", 1),
    ("EST", 1),
    ("ES", 0),
    ("ED", 0),
    ("LY", 1),
    ("ER", 1),
    ("EN", 1),
    ("TY", 1),
    ("S", 0),
    ("Y", 1),
];
// Unstressed beginnings of two-syllable words.
const PREFIXES: [&str; 34] = [
    "TRANS", "COM", "CON", "COL", "COR", "DIS", "PER", "PRE", "SUB", "SUC", "SUP", "SUR", "SUS",
    "MIS", "AB", "AC", "AD", "AF", "AP", "AS", "AT", "BE", "DE", "EM", "EN", "EX", "IM", "IN",
    "OB", "OC", "OF", "OP", "RE", "UN",
];
// Vowels weakened to AX beside the stress, vowels already weak, and vowels that make a syllable
// heavy.
const LAX: [&str; 6] = ["AE", "AA", "EH", "AH", "AO", "UH"];
const WEAK: [&str; 2] = ["AX", "IX"];
const LONG: [&str; 7] = ["IY", "EY", "AY", "OW", "UW", "AW", "OY"];

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Clone, Debug)]
pub struct Voice {
    pub sample_rate: u32,
    // Hz at the start of a sentence.
    pub pitch: f32,
    // 2.0 talks twice as fast.
    pub speed: f32,
    // Scales every resonance: below 1 a bigger mouth, above 1 a smaller one.
    pub tract: f32,
    // Widens the third formant and the fixed upper resonances, softening a bright source.
    pub width: f32,
    // Hz above which the source is low-passed, twice over; 0 leaves it whole.
    pub cutoff: f32,
    // One cycle played each pitch period in place of the built-in pulse; empty keeps the pulse.
    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(getter_with_clone))]
    pub source: Vec<f32>,
}

impl Default for Voice {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
impl Voice {
    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self {
            sample_rate: 22_050,
            pitch: 95.0,
            speed: 1.0,
            tract: 1.0,
            width: 1.0,
            cutoff: 0.0,
            source: vec![],
        }
    }

    pub fn speak(&self, text: &str) -> Vec<f32> {
        let rate = self.sample_rate as f32;
        let step = 1000.0 * self.speed.max(0.1) / rate;
        let (mut tract, mut breath, mut glottis) = ([[0f32; 2]; 6], [[0f32; 2]; 6], [0f32; 2]);
        let mut bands = [[[0f32; 2]; 2]; 7];
        let (mut now, mut phase, mut seed, mut lift, mut shape) = ([0f32; 8], 0f32, 1u32, 0f32, 0);
        // The previous mouth, radiated and aspiration samples, the last two noise samples, and
        // the low-passed noise aspiration starts from.
        let (mut mouth_was, mut radiated_was, mut breath_was) = (0f32, 0f32, 0f32);
        let (mut noise_was, mut hush) = ([0f32; 2], 0f32);
        // How nasal the voice is, and the nose's resonance.
        let (mut nasal, mut nose) = (0f32, [0f32; 2]);
        // The two low-pass stages the source passes through.
        let mut smooth = [0f32; 2];
        // Per sample: the voice, the selected noise with its shape, and aspiration with its level.
        let mut mix = vec![];
        // Summed squares of each noise and of aspiration, and of the voice in vowels with their
        // count.
        let (mut power, mut vowels) = ([0f64; 8], (0f64, 0u32));
        for (tokens, stop) in plan(text) {
            let mut sounds: Vec<([f32; 8], f32)> =
                tokens.into_iter().flat_map(|t| t.sounds).collect();
            settle(&mut sounds);
            // Every sentence ends in a pause; the contour finishes before it.
            let span: f32 = sounds.iter().rev().skip(1).map(|(s, _)| s[0]).sum();
            let mut elapsed = 0.0;
            for (target, accent) in sounds {
                if let Some(i) = SHAPES.iter().position(|s| s.0 == target[7]) {
                    shape = i;
                }
                for _ in 0..(target[0] / step) as usize {
                    let t = (elapsed / span).min(1.0);
                    elapsed += step;
                    let semitones = match stop {
                        Some('?') => 8.0 * t.powi(8),
                        Some('!') => 2.0 - 8.0 * t.powi(10),
                        _ => -8.0 * t.powi(10),
                    };
                    lift += (accent - lift) * 10.0 / rate;
                    // The pitch also drifts down three semitones across the sentence.
                    let hz = self.pitch.max(0.0) * 2f32.powf((semitones + lift - 3.0 * t) / 12.0);
                    let turn = phase + hz / rate >= 1.0;
                    phase = (phase + hz / rate).fract();
                    for (i, (now, goal)) in now.iter_mut().zip(target).enumerate().take(7).skip(1) {
                        // Per second: a fricative swells in where a release is immediate, and the
                        // voice comes in slower than it goes.
                        let pace = match i {
                            6 if goal > *now && shape < 4 => 30.0,
                            6 if goal < *now => 80.0,
                            4 if goal > *now => 60.0,
                            4 => 100.0,
                            5 | 6 => 300.0,
                            _ if target[7] == GLIDE || target[7] == BOUNDARY => 25.0,
                            _ => 60.0,
                        };
                        *now += (goal - *now) * pace / rate;
                    }
                    seed ^= seed << 13;
                    seed ^= seed >> 17;
                    seed ^= seed << 5;
                    let noise = seed as f32 / u32::MAX as f32 * 2.0 - 1.0;
                    let [_, f1, f2, f3, voicing, aspiration, frication, _] = now;
                    let goal = if target[7] == NASAL { 1.0 } else { 0.0 };
                    nasal += (goal - nasal) * 300.0 / rate;
                    let source = if self.source.is_empty() {
                        // One low-passed glottal pulse a period.
                        let pulse = if turn { voicing } else { 0.0 };
                        resonate(&mut glottis, pulse, 0.0, 100.0, rate)
                    } else {
                        // The cycle at this point of the period, between its samples.
                        let at = phase * self.source.len() as f32;
                        let (i, frac) = (at as usize % self.source.len(), at.fract());
                        let next = self.source[(i + 1) % self.source.len()];
                        voicing * (self.source[i] * (1.0 - frac) + next * frac)
                    };
                    let source = if self.cutoff > 0.0 {
                        let keep = (-TAU * self.cutoff / rate).exp();
                        smooth[0] = (1.0 - keep) * source + keep * smooth[0];
                        smooth[1] = (1.0 - keep) * smooth[0] + keep * smooth[1];
                        smooth[1]
                    } else {
                        source
                    };
                    let (k, wide) = (self.tract, self.width);
                    let formants = [
                        (f1 * k, 96.0),
                        (f2 * k, 144.0 * (1.0 + 2.0 * nasal)),
                        (f3 * k, 150.0 * (1.0 + 2.0 * nasal) * wide),
                        (3300.0 * k, 250.0 * wide),
                        (3750.0 * k, 200.0 * wide),
                        (4900.0 * k, 600.0 * wide),
                    ];
                    let through = |mem: &mut [[f32; 2]; 6], x: f32| {
                        let stages = formants.into_iter().zip(mem);
                        stages.fold(x, |x, ((f, bw), mem)| resonate(mem, x, f, bw, rate))
                    };
                    // A nasal damps the mouth's upper formants and hums through the nose.
                    let hum = resonate(&mut nose, source, 250.0 * k, 100.0, rate);
                    let mouth = through(&mut tract, source) * (1.0 - 0.5 * nasal) + nasal * hum;
                    // Radiated from the lips, then brightened.
                    let radiated = mouth - mouth_was;
                    let voice = radiated + 1.5 * (radiated - radiated_was);
                    (mouth_was, radiated_was) = (mouth, radiated);
                    if voicing > 0.9 {
                        vowels = (vowels.0 + f64::from(voice * voice), vowels.1 + 1);
                    }
                    // Noise without its lowest frequencies.
                    let edge = noise - noise_was[1];
                    noise_was = [noise, noise_was[0]];
                    // Aspiration is low-passed so radiation leaves it flat.
                    hush = noise + 0.75 * hush;
                    let breathed = through(&mut breath, hush);
                    let aspirated = breathed - breath_was;
                    breath_was = breathed;
                    power[7] += f64::from(aspirated * aspirated);
                    let mut hiss = 0.0;
                    for (i, ((_, full, _, passes), mems)) in
                        SHAPES.iter().zip(&mut bands).enumerate()
                    {
                        let shaped: f32 = passes
                            .iter()
                            .zip(mems)
                            .map(|(&(f, bw, gain), mem)| {
                                let f = if f == 0.0 {
                                    target[2].clamp(1800.0, 3200.0)
                                } else {
                                    f
                                };
                                gain * resonate(mem, edge, f, bw, rate)
                            })
                            .sum();
                        power[i] += f64::from(shaped * shaped);
                        if i == shape {
                            hiss = shaped * frication / full;
                        }
                    }
                    // The aspiration level of HH is full.
                    mix.push((voice, hiss, shape, aspirated, aspiration / 0.5));
                }
            }
        }
        let count = mix.len().max(1) as f64;
        let vowel = (vowels.0 / f64::from(vowels.1.max(1))).sqrt() as f32;
        // The gain that puts noise `i` `db` below a vowel.
        let gain = |i: usize, db: f32| {
            let rms = (power[i] / count).sqrt() as f32;
            vowel * 10f32.powf(db / 20.0) / rms.max(f32::MIN_POSITIVE)
        };
        let gains: [f32; 7] = std::array::from_fn(|i| gain(i, SHAPES[i].2));
        let breathy = gain(7, ASPIRATION);
        // All noise is low-passed at 4.5 kHz.
        let keep = (-TAU * 4500.0 / rate).exp();
        let mut dulled = 0f32;
        let out: Vec<f32> = mix
            .iter()
            .map(|&(voice, hiss, shape, aspirated, breath)| {
                let noise = hiss * gains[shape] + aspirated * breath * breathy;
                dulled = (1.0 - keep) * noise + keep * dulled;
                voice + dulled
            })
            .collect();
        let peak = out
            .iter()
            .fold(f32::MIN_POSITIVE, |peak, s| peak.max(s.abs()));
        out.iter().map(|s| s * 0.9 / peak).collect()
    }

    pub fn wav(&self, text: &str) -> Vec<u8> {
        let data: Vec<u8> = self
            .speak(text)
            .iter()
            .flat_map(|s| ((s * 32767.0).round() as i16).to_le_bytes())
            .collect();
        let len = data.len() as u32;
        let parts: [&[u8]; 13] = [
            b"RIFF",
            &(36 + len).to_le_bytes(),
            b"WAVEfmt ",
            &16u32.to_le_bytes(),
            &1u16.to_le_bytes(),
            &1u16.to_le_bytes(),
            &self.sample_rate.to_le_bytes(),
            &self.sample_rate.saturating_mul(2).to_le_bytes(),
            &2u16.to_le_bytes(),
            &16u16.to_le_bytes(),
            b"data",
            &len.to_le_bytes(),
            &data,
        ];
        parts.concat()
    }

    /// Start time in seconds of each token of `phonemes(text)`, `/` between words included and a
    /// `,` added when the text ends without a pause, then the total length.
    pub fn cues(&self, text: &str) -> Vec<f32> {
        let speed = self.speed.max(0.1);
        let (mut at, mut out) = (0.0, Vec::new());
        for (tokens, _) in plan(text) {
            for token in tokens {
                out.push(at);
                at += token.ms / 1000.0 / speed;
            }
        }
        out.push(at);
        out
    }
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
pub fn phonemes(text: &str) -> String {
    let mut out = vec![];
    let mut rest = text;
    // All-caps text is shouting, not acronyms.
    let shout = !text.contains(|c: char| c.is_ascii_lowercase());
    while let Some(c) = rest.chars().next() {
        let before = &text[..text.len() - rest.len()];
        let digits = rest.trim_start_matches(['$', '-']);
        let sign = &rest[..rest.len() - digits.len()];
        let len = if let Some((key, long)) = ABBREVIATIONS.iter().find(|(key, _)| leads(rest, key))
        {
            // St. follows a name or number when it means street.
            let saint = *key == "St."
                && !before.split_whitespace().next_back().is_some_and(|w| {
                    w.starts_with(|c: char| c.is_ascii_uppercase() || c.is_ascii_digit())
                });
            out.push(say(if saint { "SAINT" } else { long }));
            key.len()
        } else if digits.starts_with(|c: char| c.is_ascii_digit())
            && (!sign.starts_with('-') || !before.ends_with(|c: char| !c.is_whitespace()))
        {
            let end = digits
                .char_indices()
                .find(|&(i, c)| {
                    let joins = ",.:".contains(c)
                        && digits[i + 1..].starts_with(|c: char| c.is_ascii_digit());
                    !(c.is_ascii_digit() || joins)
                })
                .map_or(digits.len(), |(i, _)| i);
            let (whole, fraction) = digits[..end]
                .split_once('.')
                .unwrap_or((&digits[..end], ""));
            let after = &digits[end..];
            let percent = after.starts_with('%');
            let spell = |digits: &str| {
                let words = digits.bytes().filter(u8::is_ascii_digit);
                let words = words.map(|d| ONES[(d - b'0') as usize]);
                words.collect::<Vec<_>>().join(" ")
            };
            let ordinal = ["ST", "ND", "RD", "TH"].iter().any(|s| leads(after, s));
            let time = whole.split_once(':').is_some_and(|(_, m)| m.len() == 2);
            let mut words = match whole.replace([',', ':'], "").parse::<u64>() {
                // A time, or a year unless it ends 00 to 09: those are said as plain numbers.
                Ok(n)
                    if time
                        || whole.len() == 4
                            && fraction.is_empty()
                            && !sign.contains('$')
                            && n % 1000 >= 10 =>
                {
                    let tail = match n % 100 {
                        0 => if time { "O'CLOCK" } else { "HUNDRED" }.into(),
                        low @ 1..10 => format!("O {}", number(low)),
                        low => number(low),
                    };
                    format!("{} {tail}", number(n / 100))
                }
                Ok(n) if !whole.contains(':') => number(n),
                _ => spell(whole),
            };
            let swap = ORDINALS
                .iter()
                .find_map(|(from, to)| Some((words.strip_suffix(from)?, to)));
            if let Some((stem, to)) = swap.filter(|_| ordinal) {
                words = format!("{stem}{to}");
            }
            if !fraction.is_empty() {
                words = format!("{words} POINT {}", spell(fraction));
            }
            if sign.contains('-') {
                words = format!("MINUS {words}");
            }
            if percent {
                words += " PERCENT";
            }
            if sign.contains('$') {
                words += &format!(" DOLLAR{}", if words == "ONE" { "" } else { "S" });
            }
            out.push(say(&words));
            rest.len() - after.len() + if ordinal { 2 } else { usize::from(percent) }
        } else if c.is_ascii_alphabetic() || c == '\'' {
            let len = rest
                .find(|c: char| !c.is_ascii_alphabetic() && c != '\'')
                .unwrap_or(rest.len());
            let word = &rest[..len];
            let caps = word.bytes().all(|b| b.is_ascii_uppercase());
            let unsayable = !word.contains(VOWELS) && !word.contains('Y');
            if caps && (unsayable || len <= 4 && !shout && !little(word)) {
                out.extend(
                    word.bytes()
                        .map(|b| LETTERS[(b - b'A') as usize].to_owned()),
                );
            } else {
                out.push(say(word));
            }
            len
        } else {
            match c {
                ',' | ';' | ':' => out.push(",".into()),
                '.' | '!' | '?' => out.push(c.into()),
                _ => {}
            }
            c.len_utf8()
        };
        rest = &rest[len..];
    }
    out.retain(|p| !p.is_empty());
    // `/` parts one word from the next.
    out.iter().fold(String::new(), |said, p| {
        let join = if said.is_empty() {
            ""
        } else if p.starts_with(PAUSES) || said.ends_with(PAUSES) {
            " "
        } else {
            " / "
        };
        said + join + p
    })
}

fn leads(text: &str, key: &str) -> bool {
    text.get(..key.len())
        .is_some_and(|s| s.eq_ignore_ascii_case(key))
        && !text[key.len()..].starts_with(|c: char| c.is_ascii_alphanumeric())
}

fn say(words: &str) -> String {
    let said = words.split_whitespace().map(|word| {
        let word = word.to_ascii_uppercase();
        listed(&word).map_or_else(|| stress(&word, &rules(&word)), str::to_owned)
    });
    said.collect::<Vec<_>>().join(" / ")
}

fn listed(word: &str) -> Option<&'static str> {
    let at = words::WORDS.binary_search_by_key(&word, |&(known, _)| known);
    at.ok().map(|at| words::WORDS[at].1)
}

// A listed word said without stress, which all-caps text does not spell out.
fn little(word: &str) -> bool {
    listed(word).is_some_and(|said| !said.contains('1'))
}

// Marks the stressed vowel of `word` and weakens the short vowels beside it.
fn stress(word: &str, phonemes: &str) -> String {
    let phones: Vec<&str> = phonemes.split_whitespace().collect();
    let vowels: Vec<usize> = (0..phones.len())
        .filter(|&i| phones[i].starts_with(VOWELS))
        .collect();
    let count = vowels.len();
    let kind = |n: usize| phones[vowels[n]];
    let firm = |n: usize| !WEAK.contains(&kind(n));
    // A firm vowel other than ER, which stress placed by syllable count passes over.
    let strong = |n: usize| firm(n) && kind(n) != "ER";
    // An ending the stress ignores leaves a base word with its own count of vowels.
    let (base, core) = ADDED
        .iter()
        .find_map(|(end, added)| {
            let stem = word.strip_suffix(end).filter(|stem| stem.len() >= 3)?;
            let vowel_y = end.starts_with('Y') && stem.ends_with(VOWELS);
            (count > *added && !vowel_y).then_some((stem, count - added))
        })
        .unwrap_or((word, count));
    // A pull reaching no further back than the plain ending is part of that ending, as -ENT in
    // -MENT.
    let plain = word.len() - base.len();
    let pulled = [(word, count, plain), (base, core, 0)]
        .into_iter()
        .find_map(|(cut, size, least)| {
            let (_, back) = PULLS.iter().find(|(end, _)| {
                end.len() > least && cut.strip_suffix(end).is_some_and(|rest| rest.len() >= 2)
            })?;
            size.checked_sub(back + 1)
        });
    let wanted = if let Some(at) = pulled {
        at
    } else if LAST.iter().any(|end| word.ends_with(end)) {
        count.saturating_sub(1)
    } else if core == 2 {
        let prefixed = PREFIXES
            .iter()
            .any(|p| base.strip_prefix(p).is_some_and(|rest| rest.len() >= 3));
        usize::from((prefixed || !firm(0)) && strong(1))
    } else if core > 2 {
        let next = vowels.get(core - 1).copied().unwrap_or(phones.len());
        let heavy = LONG.contains(&kind(core - 2)) || next - vowels[core - 2] > 2;
        if heavy && strong(core - 2) {
            core - 2
        } else {
            core - 3
        }
    } else {
        0
    };
    // The firm vowel nearest the one wanted, the later on a tie.
    let at = (0..count)
        .filter(|&n| firm(n))
        .min_by_key(|&n| (n.abs_diff(wanted), n < wanted))
        .unwrap_or(0);
    let said = phones
        .iter()
        .enumerate()
        .map(|(i, p)| match vowels.iter().position(|&v| v == i) {
            Some(n) if n == at => format!("{p}1"),
            Some(n) if n.abs_diff(at) == 1 && LAX.contains(p) => "AX".into(),
            _ => (*p).to_owned(),
        });
    said.collect::<Vec<_>>().join(" ")
}

fn number(n: u64) -> String {
    let tail = |rest: u64| match rest {
        0 => String::new(),
        rest => format!(" {}", number(rest)),
    };
    if let Some((scale, name)) = SCALES.into_iter().find(|(scale, _)| n >= *scale) {
        return format!("{} {name}{}", number(n / scale), tail(n % scale));
    }
    match n as usize {
        n @ 0..20 => ONES[n].into(),
        n => format!("{}{}", TENS[n / 10], tail(n as u64 % 10)),
    }
}

fn rules(word: &str) -> String {
    let text = format!(" {word} ");
    let text = text.as_bytes();
    let (mut at, mut out) = (1, vec![]);
    while at + 1 < text.len() {
        let rule = RULES.iter().find(|(left, letters, right, _)| {
            text[at..].starts_with(letters.as_bytes())
                && fits(left.bytes().rev(), text, at as isize - 1, -1)
                && fits(right.bytes(), text, (at + letters.len()) as isize, 1)
        });
        let (len, phonemes) = rule.map_or((1, &word[at - 1..at]), |(_, letters, _, phonemes)| {
            (letters.len(), *phonemes)
        });
        out.extend(phonemes.split_whitespace());
        at += len;
    }
    out.dedup_by(|a, b| a == b && !a.starts_with(VOWELS));
    out.join(" ")
}

fn fits(context: impl Iterator<Item = u8>, text: &[u8], mut at: isize, step: isize) -> bool {
    let get = |at: isize| *text.get(at as usize).unwrap_or(&b' ');
    let vowel = |c: u8| b"AEIOUY".contains(&c);
    let consonant = |c: u8| c.is_ascii_uppercase() && !vowel(c);
    let mut context = context.peekable();
    while let Some(want) = context.next() {
        let c = get(at);
        let fits = match want {
            b' ' => !c.is_ascii_uppercase(),
            // `##` is two or more vowels.
            b'#' if context.peek() == Some(&b'#') => vowel(c),
            b'#' => {
                while vowel(get(at + step)) {
                    at += step;
                }
                vowel(c)
            }
            b':' => {
                while consonant(get(at)) {
                    at += step;
                }
                at -= step;
                true
            }
            b'^' => consonant(c),
            b'.' => b"BDVGJLMNRWZ".contains(&c),
            b'+' => b"EIY".contains(&c),
            // Left contexts only: an H counts when it ends TH, CH or SH.
            b'@' | b'&' => {
                let (letters, pairs): (&[u8], &[u8]) = match want {
                    b'@' => (b"TSRDLZNJ", b"TCS"),
                    _ => (b"SCGZXJ", b"CS"),
                };
                let pair = c == b'H' && pairs.contains(&get(at + step));
                at += if pair { step } else { 0 };
                pair || letters.contains(&c)
            }
            b'%' => ["ING", "ELY", "ER", "ES", "ED", "E"]
                .iter()
                .find(|s| {
                    text.get(at as usize..)
                        .is_some_and(|t| t.starts_with(s.as_bytes()))
                })
                .map(|s| at += s.len() as isize - 1)
                .is_some(),
            _ => c == want,
        };
        if !fits {
            return false;
        }
        at += step;
    }
    true
}

// The sounds of one phoneme: a stop is its closure and one release of noise, and the parts of
// any other compound are shortened to fit one phoneme's time.
fn sounds(name: &str) -> Vec<[f32; 8]> {
    if let Some(&(_, closure, centre, voiced)) = STOPS.iter().find(|(stop, ..)| *stop == name) {
        let full = SHAPES.iter().find(|s| s.0 == centre).map_or(0.0, |s| s.1);
        let released = sounds(closure).into_iter().flat_map(|shut| {
            let [_, f1, f2, f3, ..] = shut;
            let release = if voiced {
                [12.0, f1, f2, f3, 0.5, 0.0, full, centre]
            } else {
                [50.0, f1, f2, f3, 0.0, 0.0, full, centre]
            };
            [shut, release]
        });
        return released.collect();
    }
    match PARTS.iter().find(|(part, _)| *part == name) {
        Some((_, parts)) => {
            let mut halves: Vec<[f32; 8]> = parts
                .split(' ')
                .flat_map(sounds)
                .map(|mut s| {
                    s[0] *= 0.7;
                    s
                })
                .collect();
            if name.starts_with(VOWELS) {
                halves[1][7] = GLIDE;
            }
            halves
        }
        None if name == "/" => vec![[15.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, BOUNDARY]],
        None if name.starts_with(PAUSES) => {
            let ms = if name == "," { 150.0 } else { 400.0 };
            vec![[ms, 500.0, 1500.0, 2500.0, 0.0, 0.0, 0.0, 0.0]]
        }
        None => SOUNDS
            .iter()
            .filter(|(sound, _)| *sound == name)
            .map(|(_, s)| *s)
            .collect(),
    }
}

// Shapes an h like the vowel after it, holds the mouth through a word boundary while the voice
// eases off, and places a k or g by the vowel beside it: the closure a little above that vowel's
// second formant, where the tongue meets the palate, but never as low as a b, and the release
// there too.
fn settle(sounds: &mut [([f32; 8], f32)]) {
    for i in 0..sounds.len() {
        if sounds[i].0[5] > 0.0
            && let Some(&(vowel, _)) = sounds[i + 1..].iter().find(|s| s.0[4] > 0.9)
        {
            sounds[i].0[1..4].copy_from_slice(&vowel[1..4]);
        }
    }
    for i in 1..sounds.len() {
        if sounds[i].0[7] == BOUNDARY {
            let before = sounds[i - 1].0;
            sounds[i].0[1..5].copy_from_slice(&[before[1], before[2], before[3], before[4] * 0.5]);
        }
    }
    for i in 1..sounds.len().saturating_sub(1) {
        if sounds[i].0[7] == VELAR {
            let next = sounds[i + 1].0;
            let vowel_before = i >= 2 && sounds[i - 2].0[4] > 0.5;
            let beside = if vowel_before { sounds[i - 2].0 } else { next };
            let f2 = (beside[2] * 1.25).clamp(1800.0, 2100.0);
            sounds[i - 1].0[2..4].copy_from_slice(&[f2, 2500.0]);
            sounds[i].0[2..4].copy_from_slice(&[f2, next[3]]);
        }
    }
}

// One phoneme token of a sentence: its total length in ms and its sounds, parts expanded and
// scaled.
struct Planned {
    ms: f32,
    sounds: Vec<([f32; 8], f32)>,
}

// Sentences of text as planned phoneme tokens with each sentence's stop character for the pitch
// contour. Tokens are scanned backwards so the last vowel before a pause, and what follows it,
// is drawn out. A vowel before a voiced consonant is drawn out too, and a b, d or g starting a
// word closes briefly and silently.
fn plan(text: &str) -> Vec<(Vec<Planned>, Option<char>)> {
    let mut phonemes = phonemes(text);
    if !phonemes.is_empty() && !phonemes.ends_with(PAUSES) {
        phonemes += " ,";
    }
    let mut sentences = vec![];
    for sentence in phonemes.split_inclusive(&PAUSES[1..]) {
        let stop = sentence.chars().last().filter(|c| PAUSES[1..].contains(c));
        let mut last = true;
        let mut tokens = vec![];
        let names: Vec<&str> = sentence.split_whitespace().collect();
        for (i, token) in names.iter().enumerate().rev() {
            let (name, stressed) = token
                .strip_suffix('1')
                .map_or((*token, false), |n| (n, true));
            let (vowel, pause) = (name.starts_with(VOWELS), name.starts_with(PAUSES));
            let voiced_next = names.get(i + 1).is_some_and(|n| VOICED.contains(n));
            let opens = i
                .checked_sub(1)
                .is_none_or(|j| names[j] == "/" || names[j].starts_with(PAUSES));
            let stretch = if vowel && voiced_next { 1.5 } else { 1.0 }
                * if last && !pause { 1.4 } else { 1.0 };
            last = pause || last && !vowel;
            let (scale, accent) = match (stressed, vowel) {
                (true, _) => (1.3, 1.5),
                (_, true) => (0.8, 0.0),
                _ => (1.0, 0.0),
            };
            let mut sounds: Vec<([f32; 8], f32)> = sounds(name)
                .into_iter()
                .map(|mut s| {
                    s[0] *= stretch * scale;
                    (s, accent)
                })
                .collect();
            if opens
                && STOPS
                    .iter()
                    .any(|&(stop, .., voiced)| stop == name && voiced)
            {
                sounds[0].0[0] *= 0.5;
                sounds[0].0[4] = 0.0;
            }
            let ms = sounds.iter().map(|s| s.0[0]).sum();
            tokens.push(Planned { ms, sounds });
        }
        tokens.reverse();
        sentences.push((tokens, stop));
    }
    sentences
}

// Two-pole resonance at `f` Hz, `bw` Hz wide, passing 0 Hz unchanged.
fn resonate(mem: &mut [f32; 2], x: f32, f: f32, bw: f32, rate: f32) -> f32 {
    let r = (-PI * bw / rate).exp();
    let b = 2.0 * r * (TAU * f / rate).cos();
    let y = (1.0 - b + r * r) * x + b * mem[0] - r * r * mem[1];
    *mem = [y, mem[0]];
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spells_by_rule() {
        assert_eq!(
            phonemes("The cat, a dog. Is 2 you?"),
            "DH AX / K AE1 T , AX / D AA1 G . IH Z / T UW1 / Y UW ?"
        );
        assert_eq!(
            phonemes("I'm, I've, we're, didn't"),
            "AY1 M , AY1 V , W IY1 R , D IH1 D AX N T"
        );
    }

    #[test]
    fn spells_endings() {
        for (word, end) in [
            ("radios", "D IY OW Z"),
            ("status", "T AH S"),
            ("genius", "IY AH S"),
            ("lion", "L AY AX N"),
            ("Jane's", "N Z"),
            ("Joe's", "OW Z"),
            ("Mike's", "K S"),
            ("clothes", "TH S"),
            ("hue", "HH Y UW"),
        ] {
            assert!(rules(&word.to_ascii_uppercase()).ends_with(end), "{word}");
        }
    }

    #[test]
    fn reads_numbers() {
        for (written, spoken) in [
            ("1,250", "one thousand two hundred fifty"),
            ("21st", "twenty first"),
            ("30th", "thirtieth"),
            ("9th", "ninth"),
            ("1984", "nineteen eighty four"),
            ("1905", "nineteen o five"),
            ("2005", "two thousand five"),
            ("3:30", "three thirty"),
            ("12:00", "twelve o'clock"),
            ("3:1", "three one"),
            ("$1", "one dollar"),
            ("-$5", "minus five dollars"),
            ("3.14%", "three point one four percent"),
            ("-3 5-10", "minus three five ten"),
        ] {
            assert_eq!(phonemes(written), phonemes(spoken), "{written}");
        }
        assert_eq!(phonemes("ninth"), "N AY1 N TH");
    }

    #[test]
    fn stresses_by_shape() {
        for (word, said) in [
            ("RELATION", "R IY L EY1 SH AX N"),
            ("ELECTRIC", "IY L EH1 K T R IH K"),
            ("DEPLOY", "D AX P L OY1"),
            ("DEPLOYMENTS", "D AX P L OY1 M AX N T S"),
            ("CANTEEN", "K AX N T IY1 N"),
        ] {
            assert_eq!(stress(word, &rules(word)), said, "{word}");
        }
    }

    #[test]
    fn expands_short_forms() {
        assert_eq!(
            phonemes("Mr. Smith, 5km"),
            phonemes("mister Smith, five kilometers")
        );
        assert_eq!(phonemes("The FBI"), "DH AX / EH1 F / B IY1 / AY1");
        assert_eq!(phonemes("IT is"), phonemes("it is"));
        assert_eq!(phonemes("NO STOP HELP"), phonemes("no stop help"));
        assert_eq!(phonemes("use HTTPS"), phonemes("use H T T P S"));
        assert_eq!(phonemes("St. Louis"), phonemes("saint Louis"));
        assert_eq!(phonemes("Main St."), phonemes("Main street"));
    }

    #[test]
    fn every_phoneme_sounds() {
        let used = RULES
            .iter()
            .map(|(.., phonemes)| *phonemes)
            .chain(LETTERS)
            .chain(words::WORDS.iter().map(|(_, said)| *said))
            .flat_map(str::split_whitespace)
            .map(|name| name.trim_end_matches('1'));
        for name in used.chain([",", ".", "!", "?", "/"]) {
            assert!(!sounds(name).is_empty(), "{name}");
        }
    }

    #[test]
    fn speech_is_audible_and_bounded() {
        let samples = Voice::new().speak("Hello there! How are you?");
        assert!(samples.iter().all(|s| s.is_finite() && s.abs() <= 1.0));
        assert!(samples.iter().any(|s| s.abs() > 0.5));
        assert!(Voice::new().speak("").is_empty());
        let saw = (0..64).map(|i| 1.0 - i as f32 / 32.0).collect();
        for (knob, voice) in [
            (
                "tract",
                Voice {
                    tract: 1.2,
                    ..Voice::new()
                },
            ),
            (
                "width",
                Voice {
                    width: 2.0,
                    ..Voice::new()
                },
            ),
            (
                "cutoff",
                Voice {
                    cutoff: 2500.0,
                    ..Voice::new()
                },
            ),
            (
                "source",
                Voice {
                    source: saw,
                    ..Voice::new()
                },
            ),
        ] {
            let changed = voice.speak("Hello there! How are you?");
            assert!(
                changed.iter().all(|s| s.is_finite() && s.abs() <= 1.0),
                "{knob}"
            );
            assert!(changed.iter().any(|s| s.abs() > 0.5), "{knob}");
            assert_ne!(changed, samples, "{knob}");
        }
    }

    #[test]
    fn wav_header_matches_data() {
        let voice = Voice::new();
        let (wav, samples) = (voice.wav("hi"), voice.speak("hi").len() as u32);
        assert_eq!(wav.len() as u32, 44 + samples * 2);
        assert_eq!(&wav[..4], b"RIFF");
        assert_eq!(wav[24..28], 22_050u32.to_le_bytes());
        assert_eq!(wav[40..44], (samples * 2).to_le_bytes());
    }

    #[test]
    fn golden_speech_is_unchanged() {
        let voice = Voice::new();
        let hash = |s: &[f32]| {
            s.iter().fold(0u64, |h, x| {
                h.rotate_left(5).wrapping_add(x.to_bits() as u64)
            })
        };
        for (text, len, want) in [
            ("The cat, a dog. Is 2 you?", 59323, 0x96b1b2c3e7d78a48),
            ("Hello!", 18583, 0x801ed30fc75efcad),
            ("Judge the morning news.", 40547, 0x39922d0078be4b11),
            ("", 0, 0),
        ] {
            let samples = voice.speak(text);
            assert_eq!(samples.len(), len, "{text}");
            assert_eq!(hash(&samples), want, "{text}");
        }
        assert_eq!(
            Voice {
                speed: 2.0,
                ..Voice::new()
            }
            .speak("The cat, a dog. Is 2 you?")
            .len(),
            29657
        );
    }

    #[test]
    fn cues_track_phonemes() {
        let voice = Voice::new();
        let text = "The cat, a dog. Is 2 you?";
        let cues = voice.cues(text);
        let tokens: Vec<Planned> = plan(text).into_iter().flat_map(|(t, _)| t).collect();
        let sounds: usize = tokens.iter().map(|p| p.sounds.len()).sum();
        assert_eq!(cues.len(), tokens.len() + 1);
        assert_eq!(cues[0], 0.0);
        assert!(cues.windows(2).all(|w| w[0] <= w[1]));
        // speak floors each sound to whole samples, so the cue total can exceed the rendered length
        // by at most one sample per sound.
        let rendered = voice.speak(text).len() as f32 / voice.sample_rate as f32;
        assert!(
            (cues.last().unwrap() - rendered).abs() <= sounds as f32 / voice.sample_rate as f32
        );
        assert_eq!(voice.cues(""), vec![0.0]);
        let fast = Voice {
            speed: 2.0,
            ..Voice::new()
        };
        assert!(*fast.cues(text).last().unwrap() < *voice.cues(text).last().unwrap() / 1.9);
    }
}
