/// Convert tuple array to article array
macro_rules! articles {
    [ $( ( $a: expr, $b: expr, $c: expr $(,)? ) $(,)? )* ] => {
        &[ $( Article { headline: $a, title: $b, description: $c } ),* ]
    };
}

/// Basic article headline
pub struct Article {
    pub headline: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

/// List of article headlines
pub const ARTICLES: &[Article] = articles![
    (
        "Scientists Discover That All Food The Same, Just Different Shapes and Colors",
        "Food revelation", "According to scientists, all food is essentially the same with just varying shapes and colors.",
    ),
    (
        "Experts Recommend Getting Enough Sleep to Avoid Feeling Tired",
        "Sleep secrets", "Experts recommend getting enough sleep in order to avoid feeling tired, a revelation that may surprise many people.",
    ),
    (
        "World's Largest Rubber Bands Snapped, Causing Minor Earthquake",
        "Rubber band calamity", "The world's largest collection of rubber bands has snapped, causing a minor earthquake in the area.",
    ),
    (
        "Report: 98% of All Commas Used, Incorrectly",
        "Comma catastrophe", "A report claims that 98% of all commas are used incorrectly, leading to widespread confusion and frustration.",
    ),
    (
        "Man Shocked to Discover He's Been Eating Same Sandwich Every Day for 3 Years",
        "Sandwich shocker", "A man has discovered that he's been eating the same sandwich every day for the past three years, leaving him feeling stunned and dismayed.",
    ),
    (
        "Tip: Drink a Gallon of Milk Before Making Any Life Decisions",
        "Milk myth", "Contrary to popular belief, drinking a gallon of milk before making life decisions is not actually a helpful tip.",
    ),
    (
        "Study Finds That No One Really Knows What They're Doing",
        "Uncertainty reigns", "A new study reveals that the vast majority of people have no idea what they're doing in life.",
    ),
    (
        "News: Local Grandmother Wins National Parkour Competition",
        "Parkour prowess", "A grandmother has defied the odds and won a national parkour competition, proving that age is just a number.",
    ),
    (
        "The Only Thing More Addictive Than Social Media is Watching Paint Dry",
        "Boring addiction", "Social media has met its match in the form of watching paint dry, which experts claim is even more addictive.",
    ),
    (
        "Man Surprised to Learn His Sourdough Bread Has Its Own Instagram Account",
        "Bread branding", "A man was surprised to learn that his sourdough bread has its own Instagram account, leading him to question the boundaries of social media.",
    ),
];
