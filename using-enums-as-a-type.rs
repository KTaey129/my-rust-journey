#[derive(Debug, Clone)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Seoul,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

// return a string representation of the region's popularity
fn regions_popularity(region: &WineRegions) -> &'static str {
    match region {
        WineRegions::Bordeaux => "Highly Popular",
        WineRegions::Burgundy => "Well Known",
        WineRegions::Champagne => "Less Popular",
        WineRegions::Tuscany => "Popular",
        WineRegions::Rioja => "Moderately Popular",
        WineRegions::NapaValley => "Up and Coming",
        WineRegions::Seoul => "Prestigious",
    }
    
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };
    
    let wine3 = Wine {
        name: String::from("Soju"),
        region: WineRegions::Seoul,
    };

    println!("Wine 1: {} from {:?}, and it's {}", wine1.name, wine1.region, 
        regions_popularity(&wine1.region));
    println!("Wine 2: {} from {:?}, and it's {}", wine2.name, wine2.region,
        regions_popularity(&wine2.region);
    println!("Wine 3: {} from {:?}, and it's {}", wine3.name, wine3.region,
        regions_popularity(&wine3.region));
    
    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);

}

// Clone or Borrow wine1.region
// You have two options:
//         1. Borrow the Region Instead of Moving: Pass a reference (&wine1.region) to regions_popularity instead of moving the value:

// println!(
//     "Wine 1: {} from {:?}, and it's {}",
//     wine1.name,
//     wine1.region,
//     regions_popularity(&wine1.region) // Pass by reference
// );

// Update regions_popularity to accept a reference:

// fn regions_popularity(region: &WineRegions) -> &'static str {
//     match region {
//         WineRegions::Bordeaux => "Highly Popular",
//         WineRegions::Tuscany => "Moderately Popular",
//         _ => "Unknown",
//     }
// }
// 				2. Clone the Region: If you need to move the value multiple times, you can clone it:


// println!(
//     "Wine 1: {} from {:?}, and it's {}",
//     wine1.name,
//     wine1.region,
//     regions_popularity(wine1.region.clone()) // Clone the value
// );

// This works only if WineRegions implements the Clone trait.
// To derive Clone for the enum:


// #[derive(Clone)]
// enum WineRegions {
//     Bordeaux,
//     Tuscany,
//     Unknown,
// }
