use rand::seq::SliceRandom;
use std::io;

fn main() {
    // Define the recommendations for each language level
    let messages = vec![
        ("beginner", vec![
            "Start talking with other native speakers for at least 30 minutes a day",
            "Start watching your favorite films and TV series in this language",
            "Start learning the language using different apps",
            "Start singing along with subtitled songs",
            "Start storytelling a picture"
        ]),
        ("intermediate", vec![
            "Start describing your day",
            "Start translating famous phrases in this language",
            "Start joining forums in this language",
            "Start writing short posts or comments daily in the language",
            "Start listening to short podcast episodes"
        ]),
        ("advanced", vec![
            "Start reading new books in this language",
            "Start finding new friends by speaking this language",
            "Start learning new disciplines in a new language",
            "Start creative writing",
            "Start recording your speech"
        ]),
    ];

    // Display the language level options
    println!("Choose your language level:");
    println!("1. Beginner");
    println!("2. Intermediate");
    println!("3. Advanced");

    // Get user input for language level
    let mut level = String::new();
    io::stdin().read_line(&mut level).expect("Failed to read input");
    let level = level.trim();

    // Map the user input to the corresponding level
    let level = match level {
        "1" => "beginner",
        "2" => "intermediate",
        "3" => "advanced",
        _ => {
            println!("Invalid choice. Please select 1, 2, or 3.");
            return;
        }
    };

    // Find the recommendations for the selected level
    let recommendations = messages.iter()
        .find(|(lvl, _)| *lvl == level)
        .map(|(_, recs)| recs)
        .unwrap_or_else(|| {
            println!("No recommendations found for the selected level.");
            &vec![]
        });

    // Display a random recommendation
    if let Some(random_message) = recommendations.choose(&mut rand::thread_rng()) {
        println!("Recommendation: {}", random_message);
    } else {
        println!("No recommendations available for the selected level.");
    }
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
