mod libs{
    mod io_lib;
}
use std::{thread, time};

use std::thread::sleep;

use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION as G;

mod particle;
use particle::{Particle, BidimensionalVector};

fn main() {
    let earth_mass = 5.97*(10.0_f64.powf(24.0));
    let fixed_particle: Particle = Particle::new(earth_mass, 0.0, 0.0, 0.0, true);
    let mut moving_particle: Particle = Particle::new(1.0, 0.0, 0.0, 6370000.0, false);
    while true {
        moving_particle.force = moving_particle.calc_gravity_force(fixed_particle);
        //println!("{}", moving_particle.force.get_module());
        moving_particle.sim_time(0.1);
        moving_particle.print_particle_info();
        sleep(time::Duration::from_millis(100));

    }
}

pub fn calc_gravity_force(first_particle: Particle, second_particle: Particle) -> BidimensionalVector{
    let distance: f64 = first_particle.position.distance_vector_module(second_particle.position);
    if distance != 0.0{
        let force = G*first_particle.mass*second_particle.mass/(distance).powf(2.0);
        return first_particle.position.distance_vector(second_particle.position).unitary_vector().multiply_vector_by_f64(force);
    }else{
        return BidimensionalVector::new(0.0, 0.0);
    }
}
