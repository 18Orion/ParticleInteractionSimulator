use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION as G;

#[derive(Copy, Clone)]
pub struct Particle{
    pub mass: f64,
    pub charge: f64,
    pub force: BidimensionalVector,
    pub velocity: BidimensionalVector,
    pub position: BidimensionalVector,
    pub fixed: bool
}

/*Bidimensional vector struct */
#[derive(Copy, Clone)]
pub struct BidimensionalVector{
    x: f64,
    y: f64,
    module: f64
}

impl Particle {
    pub fn new(particle_mass: f64, particle_charge: f64, x_position: f64, y_position: f64, is_particle_fixed: bool) -> Particle{
        return Particle{
            mass: particle_mass,
            charge: particle_charge,
            force: BidimensionalVector::new(0.0, 0.0),
            velocity: BidimensionalVector::new(0.0, 0.0),
            position: BidimensionalVector::new(x_position, y_position),
            fixed: is_particle_fixed
        };
    }

    pub fn sim_time(&mut self, time: f64){
        let mut acceleration: BidimensionalVector = self.force.multiply_vector_by_f64(self.mass);
        self.velocity = self.velocity.add_vector(acceleration.multiply_vector_by_f64(time));
        self.position = self.position.add_vector(self.velocity.multiply_vector_by_f64(time));

    }

    pub fn print_particle_info(&mut self){
        print!("Position: {}", self.position.get_module());
        print!("\t\tVelocity: {}", self.velocity.get_module());
        print!("\t\tForce: {}", self.force.get_module());
        println!("")
    }

    pub fn get_distance(&mut self, second_particle: Particle) -> f64{
        let distance_vector: BidimensionalVector = self.position.distance_vector(second_particle.position);
        let distance: f64 = distance_vector.get_module();
        return distance;
    }

    pub fn calc_gravity_force(&mut self, second_particle: Particle) -> BidimensionalVector{
        let distance_vector: BidimensionalVector = self.position.distance_vector(second_particle.position);
        let distance: f64 = distance_vector.get_module();
        let mut unitary_vector: BidimensionalVector = distance_vector.unitary_vector();
        let force: f64 = (G*self.mass*second_particle.mass)/(distance.powf(2.0));
        //println!("{}", distance);
        return unitary_vector.multiply_vector_by_f64(force);
    }

    pub fn calc_gravity_field_force(&mut self, particles: Vec<Particle>){
        if !(self.fixed){
            let mut total_force: BidimensionalVector = BidimensionalVector::new(0.0, 0.0);
            //Iterates in a array of particles
            for mut affecting_particle in particles{
                //Calculates the force acting uppon the particle
                let particle_force = self.calc_gravity_force(affecting_particle);
                //Adds the force to the total force summing it as a vector
                total_force = total_force.add_vector(particle_force);
            }
            self.force = total_force;       //Sets the acting force on the particle
        }else{
            self.force = BidimensionalVector::new(0.0, 0.0);    //Sets to zero the acting force
        }
    }
}


impl BidimensionalVector {
    pub fn new(vector_x: f64, vector_y: f64) -> BidimensionalVector{
        return BidimensionalVector{
            x: vector_x,
            y: vector_y,
            module: BidimensionalVector::calc_vector_module(vector_x, vector_y)
        };
    }

    pub fn get_x(self) -> f64{
        return self.x;
    }

    pub fn get_y(self) -> f64{
        return self.y;
    }

    pub fn get_module(self) -> f64{
        return self.module;
    }

    pub fn calc_vector_module(vector_x: f64, vector_y: f64) -> f64{
        return (vector_x.powf(2.0)+vector_y.powf(2.0)).sqrt();
    }

    pub fn calc_own_module(&mut self){
        self.module = (self.x.powf(2.0)+self.y.powf(2.0)).sqrt();
    }

    pub fn multiply_vector_by_f64(&mut self, multiplier: f64) -> BidimensionalVector{
        let new_x = self.x*multiplier;
        let new_y = self.y*multiplier;
        return BidimensionalVector::new(new_x, new_y);
    }
    pub fn unitary_vector(self) -> BidimensionalVector{
        return BidimensionalVector::new(self.x/self.module, self.y/self.module);
    }
    pub fn add_vector(self, second_vector: BidimensionalVector) -> BidimensionalVector{
        return BidimensionalVector::new(self.x+second_vector.x, self.y+second_vector.y);
    }

    pub fn print_vector(self){
        print!("{}, {}", self.x, self.y)
    }

    pub fn distance_vector(self, second_vector: BidimensionalVector) -> BidimensionalVector{
        return BidimensionalVector::new(second_vector.x-self.x, second_vector.y-self.y);
    }

    pub fn distance_vector_module(self, second_vector: BidimensionalVector) -> f64{
        return ((self.x-second_vector.x).powf(2.0)+(self.y-second_vector.y).powf(2.0)).sqrt();
    }
}
