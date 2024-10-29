struct Planet
{
    name: String,
    description: String,
    composition: String,
    gravity: f32,
    diameter: f32,
    mass: f32,
    temperature: f32,
    habitable: bool,
}

impl Planet 
{
    fn real_planet(planet_name: &str) -> Planet
    {
        let planet = match planet_name 
        {
            "Mercury" => Planet 
            {
                name: "Mercury".to_string(),
                description: "The smallest planet in our solar system.".to_string(),
                composition: "Rocky".to_string(),
                diameter: 4879.0,
                mass: 3.3011e23,
                temperature: 167.0,
                habitable: false,
            },
            "Venus" => Planet 
            {
                name: "Venus".to_string(),
                description: "The second planet from the Sun.".to_string(),
                composition: "Rocky".to_string(),
                diameter: 12104.0,
                mass: 4.8675e24,
                temperature: 464.0,
                habitable: false,
            },
            "Earth" => Planet 
            {
                name: "Earth".to_string(),
                description: "The third planet from the Sun and the only astronomical object known to harbor life.".to_string(),
                composition: "Iron, Oxygen, Silicon, Magnesium, Sulfur, Nickel, Calcium, Aluminum".to_string(),
                diameter: 12742.0,
                mass: 5.972e24,
                temperature: 15.0,
                habitable: true,
            },
            "Mars" => Planet 
            {
                name: "Mars".to_string(),
                description: "The fourth planet from the Sun.".to_string(),
                composition: "Rocky".to_string(),
                diameter: 6779.0,
                mass: 6.4171e23,
                temperature: -65.0,
                habitable: false,
            },
            "Jupiter" => Planet 
            {
                name: "Jupiter".to_string(),
                description: "The largest planet in our solar system.".to_string(),
                composition: "Gas Giant".to_string(),
                diameter: 139820.0,
                mass: 1.8982e27,
                temperature: -110.0,
                habitable: false,
            },
            "Saturn" => Planet 
            {
                name: "Saturn".to_string(),
                description: "The sixth planet from the Sun, known for its ring system.".to_string(),
                composition: "Gas Giant".to_string(),
                diameter: 116460.0,
                mass: 5.6834e26,
                temperature: -140.0,
                habitable: false,
            },
            "Neptune" => Planet
            {
                name: "Neptune".to_string(),
            },
            "Pluto" => panic!("Sorry! Pluto isn't a planet"),
            _ => panic!("Unknown planet name"),
        };

        return planet;
    }

    fn planet_generator()
    {

    }
}

