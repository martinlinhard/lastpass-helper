mod lp_helper;
mod rofi_helper;

use structopt::StructOpt;
use crate::lp_helper::LPHelper;

#[derive(Debug, StructOpt)]
#[structopt(name = "Lastpass Helper", about = "A simple app for accessing lastpass.")]
struct Opt {
    /// Copy password to clipboard
    #[structopt(short, long)]
    pub pw: bool,
    /// Copy username to clipboard
    #[structopt(short, long)]
    pub username: bool,
}

fn main() {
    let opt = Opt::from_args();
    if (!opt.pw && !opt.username) || (opt.pw && opt.username) {
        println!("invalid!");
        return;
    }
    
    let mut pw_helper = LPHelper::new();
    pw_helper.get_account();

    if opt.pw {
        // Copy password
        pw_helper.copy_password();
    }
    else {
        // Copy username
        pw_helper.copy_username();
    }
}
