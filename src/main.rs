mod particle;
mod sim;
mod libs{
    mod io_lib;
}

use std::{time};
use std::thread::sleep;
use particle::{BidimensionalVector, Particle};
use sim::Simulator;

fn main() {
    //Earth vars
    let  earth_mass: f64 = 5.97*(10.0_f64.powf(24.0));
    const  earth_radius: f64= 6370000.0;
    let mut sim: Simulator = Simulator::new(0.0001);
    sim.add_particle(sim::Particle::new(earth_mass+10000.0, 0.0, earth_radius, 0.0, true));
    sim.add_particle(sim::Particle::new(earth_mass, 0.0, -earth_radius, 0.0, true));
    sim.add_particle(sim::Particle::new(1.0, 0.0, 100.0, 0.0, false));
    while true{
        sim.simulate_during_seconds(0.01);
        print_particle_info(sim.particle_array[2]);
        sleep(time::Duration::from_millis(100));    
    }
}

pub fn print_particle_info(part: crate::sim::Particle){
    println!("Posición: {}\tVelocidad: {}\t Fuerza: {}", 
        part.position.get_module(), 
        part.velocity.get_module(), 
        part.force.get_module());
}
