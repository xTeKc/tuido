#[allow(unused)]
use ctrlc::*;
use ncurses::*;

#[tokio::main]
async fn main() {
    initscr();

    // creating left window;
    let left_win = newwin(20, 22, 1, 1);
    refresh();

    // making box border with default border styles
    box_(left_win, 0, 0);

    // move and print in left window
    mvwprintw(left_win, 0, 1, "Dos");
    mvwprintw(left_win, 1, 1, "Do 1");

    // refreshing the left window
    wrefresh(left_win);

    // creating right window;
    let right_win = newwin(20, 54, 1, 25);
    refresh();

    // making box border with default border styles
    box_(right_win, 0, 0);

    // move and print in right window
    mvwprintw(right_win, 0, 1, "Checks");
    mvwprintw(right_win, 1, 1, "Check 1");

    // refreshing the right window
    wrefresh(right_win);

    // 'q' to quit
    let mut quit = false;
    while !quit {
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            _ => (),
        }
    }
    endwin();
}
