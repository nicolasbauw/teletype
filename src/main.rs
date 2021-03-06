use std::{ env, error::Error, process, thread, sync::mpsc, io::stdout, io::Write, fs, time };
use intel8080::{CPU, memory::ROMSpace};
use console::{Term, Key, style};

fn main() {
    if let Err(e) = load_execute() {
        println!("{}", e);
        process::exit(1);
    }
}

fn load_execute() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    let term = Term::stdout();
    let  mut a = env::args();
    let mut c = CPU::new();
    /* This byte of ROM at the end of address space is there to meet basic 3.2 initialization code requirement
    otherwise automatic RAM detection routine loops forever */
    c.bus.rom_space = Some(ROMSpace{start: 0xffff, end: 0xffff});
    c.set_cb_out(out_callback);

    // Loads assembled program into memory
    if let Some(f) = a.nth(1) {
        c.bus.load_bin(&f, 0x0)?;
    } else {
        println!("No file specified");
        process::exit(1);
    }

    // Setting up Altair switches for 88-SIO (4K BASIC 3.2)
    c.bus.set_io_in(255, 0x00);

    // Since the console crate read key function is blocking, we spawn a thread
    thread::spawn(move || {
        loop {
            if let Some(ch) = getch(&term, &tx) {
                tx.send(ch).unwrap()
            }
        } 
    });

    loop {
        c.execute_slice();

        // Will likely never happen. There just to meet function return type requirement.
        if c.pc == 0xffff { return Ok(()) };

        if let Ok(ch) = rx.try_recv() {
            c.bus.set_io_in(0, 0);
            c.bus.set_io_in(1, ch);
        }
    }
}

fn getch(term: &console::Term, tx: &std::sync::mpsc::Sender<u8>) -> Option<u8> {
    match term.read_key() {
        Ok(k) => match k {
            Key::Char(c) => Some(c as u8),
            Key::Enter => Some(0x0d),
            Key::Escape => {
                if let Err(e) = toggle_menu(term, tx) { println!("{}", e) };
                return None
            },
            _ => None
        },
        Err(_) => None
    }
}

fn toggle_menu(term: &console::Term, tx: &std::sync::mpsc::Sender<u8>) -> Result<(), Box<dyn Error>> {
    let delay = time::Duration::from_millis(20);
    term.move_cursor_to(0, 0)?;
    term.clear_screen().unwrap();
    println!("{}uit\t{}oad", style("[Q]").magenta(), style("[L]").magenta());
    loop {
        match term.read_key()? {
            Key::Escape => { term.clear_screen().unwrap(); return Ok(())},
            Key::Char('Q') => { process::exit(0) },
            Key::Char('L') => {
                term.clear_screen()?;
                term.write_line("File ? ")?;
                let file = term.read_line()?;
                let bas = fs::read_to_string(file)?;
                for line in bas.lines() {
                    for c in line.chars() {
                        tx.send(c as u8)?;
                        thread::sleep(delay);
                    }
                    tx.send(0x0d)?;
                    thread::sleep(delay*10);
                }
                return Ok(());
            }
            Key::Char('C') => {
                tx.send(0x03)?;
            }
            _ => {}
        }
    }
}

fn out_callback(c: &mut CPU, device: u8, data: u8) -> Option<u8> {
    // Data sent to device 1 (OUT) ? we display it
    if device == 1 {
        let value = data & 0x7f;
        if value >= 32 && value <=125 || value == 0x0a || value == 0x0d {
            print!("{}", value as char);
            stdout().flush().ok();
            // Clearing IO in to be ready for next key press
            c.bus.set_io_in(0, 1);
        }
    }
    None
}