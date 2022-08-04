use crate::{infoln};
use clap::Args;
use crate::utils::{print::*};
use colored::Colorize;

#[derive(Args)]
pub struct Door {
    /// Narration speed; default norm
    #[clap(short, long, arg_enum, value_name = "SPEED")]
    narrate_speed: Option<NarrateSpeed>,
    
    /// Narration speed; Default norm
    #[clap(short, long, value_parser)]
    wrapping: Option<usize>,
}

const IPSUM: &str = 

"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris posuere auctor libero ac vulputate. Nunc a mollis dolor. Proin malesuada a turpis non imperdiet. 
";   




pub fn process_door(door: &Door) {
    
    infoln!("Opening Door...");

    let speed = match &door.narrate_speed {
        Some(s) => s.clone(),
        _ => NarrateSpeed::Norm
    };

    let wrap = match &door.wrapping {
        Some(v) => *v,
        _ => 75usize
    };

    narrate(IPSUM, speed, wrap);
    
    infoln!("Done");

}