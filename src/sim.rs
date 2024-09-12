#[path = "./particle.rs"]
mod particle;
pub use particle::Particle;
use physical_constants::ALPHA_PARTICLE_MASS;


pub struct Simulator{
    time_elapsed: f32,
    time_gap: f32,
    pub particle_array: Vec<Particle>
}

impl Simulator {
    pub fn new(gap: f32) -> Simulator{
        return Simulator {
            time_elapsed: 0.0,
            time_gap: gap,
            particle_array: vec![]
        };
    }

    pub fn simulate_during_seconds(&mut self, seconds: f32){
        let initial_time = self.time_elapsed;
        while self.time_elapsed-initial_time < seconds{
            self.simulate()
        };
    }

    fn simulate(&mut self){
        self.time_elapsed+=self.time_gap;           //Adds the time
        for particle_index in 0..self.particle_array.len(){ 
            //Copies the list and removes the particle we are acting upon
            let mut particle_array_copy: Vec<Particle> = self.particle_array.clone();  
            particle_array_copy.remove(particle_index);
            //Calculates the force of the gravity field
            self.particle_array[particle_index].calc_gravity_field_force(particle_array_copy);
        }
        for particle_index in 0..self.particle_array.len(){
            self.particle_array[particle_index].sim_time(self.time_gap as f64)
        }
    }

    pub fn add_particle(&mut self, particle: Particle){
        self.particle_array.push(particle);
    }
}