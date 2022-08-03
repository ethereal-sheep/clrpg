use crate::{errln, infoln, warnln};
use clap::Args;
use crate::utils::{common::*, print::*};
use colored::Colorize;

#[derive(Args)]
pub struct Door {
    // /// Say yes
    // #[clap(short, long, action)]
    // yes: bool,
}

const IPSUM: &str = 

"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris posuere auctor libero ac vulputate. Nunc a mollis dolor. Proin malesuada a turpis non imperdiet. 
";   




pub fn process_door(_door: &Door) {
    
    infoln!("Opening Door...");

    narrate(IPSUM, NarrateSpeed::Fast, 75usize);
    
    infoln!("Done");

}