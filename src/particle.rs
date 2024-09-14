use core::time;

use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION as G;

#[derive(Copy, Clone)]
pub struct Particle{
    pub mass: f64,
    pub charge: f64,
    pub radius: f64,
    pub acceleration: BidimensionalVector,
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
    pub fn new(particle_mass: f64, particle_charge: f64, particle_radius: f64, initial_position: Option<BidimensionalVector>, initial_velocity: Option<BidimensionalVector>, is_particle_fixed: Option<bool>) -> Particle{
        return Particle{
            mass: particle_mass,
            charge: particle_charge,
            radius: particle_radius,
            acceleration: BidimensionalVector::new(0.0, 0.0),
            velocity: initial_velocity.unwrap_or(BidimensionalVector::new(0.0, 0.0)),
            position: initial_position.unwrap_or(BidimensionalVector::new(0.0, 0.0)),
            fixed: is_particle_fixed.unwrap_or(false)
        };
    }

    pub fn sim_forces(&mut self, time: f64, particles: Vec<Particle>){
        if !(self.fixed){
            self.acceleration = self.calc_gravity_field_acceleration(&particles);
            let change_in_velocity = self.calc_change_in_velocity(self.acceleration, 
                time);  
            self.velocity = self.velocity.add_vector(change_in_velocity);
        }
    }

    pub fn sim_time(&mut self, time: f64){
        self.position = self.move_particle(time);
    }

    fn move_particle(&mut self, time: f64) -> BidimensionalVector{
        return self.position.add_vector(self.velocity.multiply_vector_by_f64(time));
    }

    pub fn get_distance(&mut self, second_particle: Particle) -> f64{
        return self.position.distance_vector(second_particle.position)
            .get_module();
    }

    pub fn is_going_to_collision(mut self, second_particle: Particle) -> bool{
        if self.get_distance(second_particle)<=(self.radius+second_particle.radius){
            return true;
        }else {
            return false;
        }
    }

    pub fn calc_gravity_field_force(&mut self, particles: &Vec<Particle>) -> BidimensionalVector{
        return  self.calc_gravity_field_acceleration(particles).multiply_vector_by_f64(self.mass);       //Sets the acting force on the particle
    }

    pub fn calc_gravity_field_acceleration(&mut self, particles: &Vec<Particle>) -> BidimensionalVector{
        let mut total_acceleration: BidimensionalVector = BidimensionalVector::new(0.0, 0.0);
        //Iterates in a array of particles
        for mut affecting_particle in particles{
            //Calculates the force acting uppon the particle
            if !(self.is_going_to_collision(*affecting_particle)){
                //Adds the force to the total force summing it as a vector
                total_acceleration = total_acceleration.add_vector(self.calc_gravity_acceleration(*affecting_particle));
            }else {
                println!("Collision...");
                self.velocity = BidimensionalVector::new(0.0, 0.0);
                return BidimensionalVector::new(0.0, 0.0);
            }
        }
        return  total_acceleration;       //Sets the acting force on the particle
    }

    fn calc_change_in_velocity(&mut self, mut acceleration: BidimensionalVector, time: f64) -> BidimensionalVector{
        return acceleration
            .multiply_vector_by_f64(time)
    }

    fn calc_gravity_potential_difference(&mut self, particles: &Vec<Particle>, time: f64) -> f64{
        let second_position = self.move_particle(time);
        let mut initial_potential: f64 = 0.0;
        let mut final_potential: f64 = 0.0;
        for mut affecting_particle in particles{
            initial_potential+=self.calc_gravitational_potential(*affecting_particle, self.position);
            final_potential+=self.calc_gravitational_potential(*affecting_particle, second_position);

        }
        return initial_potential-final_potential;
    }

    fn calc_gravitational_potential(&mut self, second_particle: Particle, position: BidimensionalVector) -> f64{
        let distance: f64 = position.distance_vector(second_particle.position)
            .get_module();
        return -G*second_particle.mass/distance;
    }

    fn calc_gravity_force(&mut self, second_particle: Particle) -> BidimensionalVector{
        let distance_vector: BidimensionalVector = self.position.distance_vector(second_particle.position);
        let distance: f64 = distance_vector.get_module();
        let mut unitary_vector: BidimensionalVector = distance_vector.unitary_vector();
        let force: f64 = (G*self.mass*second_particle.mass)/(distance.powf(2.0));
        return unitary_vector.multiply_vector_by_f64(force);
    }

    fn calc_gravity_acceleration(&mut self, second_particle: Particle) -> BidimensionalVector{
        let distance_vector: BidimensionalVector = self.position.distance_vector(second_particle.position);
        let distance: f64 = distance_vector.get_module();
        let mut unitary_vector: BidimensionalVector = distance_vector.unitary_vector();
        let acceleration: f64 = (G*second_particle.mass)/(distance.powf(2.0));
        return unitary_vector.multiply_vector_by_f64(acceleration);
    }

    pub fn new_particle_from_reference(self, particle_mass: f64, 
            particle_charge: f64, 
            particle_radius: f64, 
            initial_position: Option<BidimensionalVector>, 
            initial_velocity: Option<BidimensionalVector>, 
            is_particle_fixed: Option<bool>) -> Particle{
        return Particle{
            mass: particle_mass,
            charge: particle_charge,
            radius: particle_radius,
            acceleration: BidimensionalVector::new(0.0, 0.0),
            velocity: initial_velocity.unwrap_or(BidimensionalVector::new(0.0, 0.0)).add_vector(self.velocity),
            position: initial_position.unwrap_or(BidimensionalVector::new(0.0, 0.0)).add_vector(self.position),
            fixed: is_particle_fixed.unwrap_or(false)
        };
    }

    pub fn new_particle_in_stable_orbit(self, particle_mass: f64, 
        particle_charge: f64, 
        particle_radius: f64,
        distance_from_radius: f64) -> Particle {
            return Particle{mass: particle_mass,
            charge: particle_charge,
            radius: particle_radius,
            acceleration: BidimensionalVector::new(0.0, 0.0),
            velocity: BidimensionalVector::new(0.0, 
                (G*self.mass/(self.radius+distance_from_radius)).sqrt())
                .add_vector(self.velocity),
            position: BidimensionalVector::new(self.radius+distance_from_radius, 
                0.0).add_vector(self.position),
            fixed: false};
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

    pub fn to_string(self) -> String{
        return self.x.to_string()+", "+&self.y.to_string();
    }

    pub fn distance_vector(self, second_vector: BidimensionalVector) -> BidimensionalVector{
        return BidimensionalVector::new(second_vector.x-self.x, second_vector.y-self.y);
    }

    pub fn distance_vector_module(self, second_vector: BidimensionalVector) -> f64{
        return ((self.x-second_vector.x).powf(2.0)+(self.y-second_vector.y).powf(2.0)).sqrt();
    }

    pub fn divide_by_f64(&mut self, divisor: f64) -> BidimensionalVector{
        return self.multiply_vector_by_f64(1.0/divisor);
    }
}
