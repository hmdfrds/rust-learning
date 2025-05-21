trait Describable {
    fn describe(&self) -> String;
}

struct Planet {
    name: String,
    diameter: f64,
    has_atmosphere: bool,
}

struct Star {
    name: String,
    star_type: String,
    temperature: u32,
}

impl Describable for Planet {
    fn describe(&self) -> String {
        format!(
            "{} is a planet with a diameter of {} km. Atmosphere: {}.\n",
            self.name,
            self.diameter,
            if self.has_atmosphere { "yes" } else { "no" },
        )
    }
}

impl Describable for Star {
    fn describe(&self) -> String {
        format!(
            "{} is a {} star with a surface temperature of {} K.\n",
            self.name, self.star_type, self.temperature
        )
    }
}

fn print_description<T: Describable>(item: &T) {
    println!("{}", item.describe());
}

fn main() {
    let earth = Planet {
        name: "earth".to_string(),
        diameter: 30000000.0,
        has_atmosphere: true,
    };
    let sun = Star {
        name: "Sun".to_string(),
        star_type: "Red Giant".to_string(),
        temperature: 400020,
    };

    print_description(&earth);
    print_description(&sun);

    let bodies: Vec<Box<dyn Describable>> = vec![Box::new(earth), Box::new(sun)];
    for body in bodies {
        println!("{}", body.describe());
    }
}
