struct Planet
{
    name: String,
    description: String,
    composition: String,
    diameter: f32,
    mass: f32,
    temperature: f32,
    habitable: bool,
}

impl Planet 
{
    fn real_planet(planet_name: &str) -> Planet
    {
        let planet = match planet_name {
            "Mercury" => Planet 
            {
                name: "Mercury".to_string(),

            },
            "Venus" => Planet 
            {
                name: "Venus".to_string(),
            },
            "Earth" => Planet
            {

            },
            "Mars" => Planet
            {

            },
            "Jupiter" => Planet
            {

            },
            "Saturn" => Planet
            {

            },
            "Uranus" => Planet
            {

            },
            "Neptune" => Planet
            {

            },
            "Pluto" => panic!("Sorry! Pluto is not a real planet!"),
            _ => panic!("Unknown planet in our solar system")
            
        };

        return planet;
    }

    fn planet_generator() -> Planet
    {

    }
}

