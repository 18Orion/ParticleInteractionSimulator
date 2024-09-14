mod particle;
mod sim;
mod libs{
    mod io_lib;
}

use std::{time};
use std::thread::sleep;
use sim::particle::BidimensionalVector;
//use particle::{BidimensionalVector, Particle};
use sim::Simulator;

fn main() {
    //Earth vars
    let gap: f32 = 1.0;
    let sim_time: i32 = 1;
    let mut i = 0.0;
    let  earth_mass: f64 = 5.97*(10.0_f64.powf(24.0));
    const  earth_radius: f64= 6370000.0;
    let moon_mass: f64 = 7.349*(10.0_f64.powf(22.0));
    const moon_orbital_radius: f64 = 398455710.0;
    const moon_radius: f64 = 1737400.0;
    let mut sim: Simulator = Simulator::new(gap);
    sim.add_particle(sim::Particle::new(earth_mass,
        0.0,
        earth_radius, 
        Some(BidimensionalVector::new(0.0, 0.0)),
        Some(BidimensionalVector::new(0.0, 0.0)),
        Some(true)));
    sim.add_particle(sim::Particle::new(moon_mass,  //Luna
        0.0, 
        moon_radius,
        Some(BidimensionalVector::new( moon_orbital_radius, 0.0)), 
        Some(BidimensionalVector::new(0.0, 1000.0)),
        Some(false)));
    sim.add_particle(sim::Particle::new(2.0, 
        0.0, 
        1.0,
        Some(BidimensionalVector::new(0.0, earth_radius+400000.0)), 
        Some(BidimensionalVector::new(7670.0, 0.0)),
        None));
    
    sim.add_particle(sim.particle_array[1].new_particle_in_stable_orbit(10.0, 
        0.0, 
        1.0,
        1000.0));
    while true{
        print_particle_info(sim.particle_array[3], i);
        sim.simulate_n_times(sim_time);
        i+=gap as f64 * sim_time as f64;
        sleep(time::Duration::from_millis(10));    
    }
}

pub fn print_particle_info(part: crate::sim::Particle, second: f64){
    print!("\rSegundo: {}\tPosición: {}\tVelocidad: {}\t Aceleración: {}", 
        second,
        part.position.get_module()-6370000.0, 
        part.velocity.get_module(), 
        part.acceleration.get_module());
}
