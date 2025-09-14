use std::thread::sleep;
use std::time::Duration;

let immutable = "I am an immutable message, this means that it cannot be changed!";
let mut mutable = "But I am immutable's evil twin, I do the EXACT OPPOSITE of immutable.";
println!(immutable);
println!(mutable);
sleep(Duration::from_secs(2));
mutable = "See? I just changed myself after two seconds!"
println!(mutable);

// What I did:
// 1. I imported the sleep function & duration function in order to add timed delays.
// 2. I declared both immutable & mutable variables.
// 3. I initialized the sleep and duration function for a 2-second delay.
// 4. I changed the mutable variable's value and declared the new mutable value.
