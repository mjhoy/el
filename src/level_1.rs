use ncurses::*;
use std::char;
use std::thread::sleep;
use std::time::Duration;
use map::*;

static LEVEL_1: &'static str =
    concat!("                  ",
            "   #########      ",
            "   #       ###### ",
            "   #      0|    # ",
            "   #       ###  # ",
            "   # L     # #  # ",
            "   ######### #--# ",
            "             #### ");

fn event0(m: &mut Map, event_pos: usize) -> bool {
    clear();
    printw("Let's open the door!");
    refresh();
    sleep(Duration::new(1,0));
    printw("\nPress ANY key to continue.");
    getch();

    m.pos = event_pos;
    // m.data[m.pos + 1] = ' ';
    // m.data[m.pos] = ' ';
    return true;
}

pub fn run_level_1() {
    // let mut map = Map::new(LEVEL_1.to_string(), 18, 8, vec![event0]);
    let mut map = map_from_str(LEVEL_1_);

    map.render();

    let mut keep_playing: bool = true;

    while keep_playing == true {
        let ich = getch() as u32;
        let maybe_ch = char::from_u32(ich);

        let ch = match maybe_ch {
            Some(ch) => ch,
            None     => {
                warn!("Unknown character: {}", ich);
                'q'
            }
        };

        if ch == 'q' || ch == 'Q' {
            keep_playing = false;
        } else {
            map.move_pos(ch);
            map.render();
        }
    }
}

const LEVEL_1_: &'static str = r"

 #########
 #       ########################
 #   L   |        |             #
 #       #######--####          #
 #########   #    #  #          #
             #    #  #####---####
             ######      #   #
                         #   #
             #######     #   #
             #     #######   #
             #     |         #
             #################
";
