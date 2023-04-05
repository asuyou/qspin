use std::{thread, time};
use std::io::stdout;
use std::io::Write;
use quat::Quat;

mod quat;

const S_WIDTH: i32 = 100;
const S_HEIGHT: i32 = 50;

const S_DIST: f64 = 20.0;
const OBJ_DIST: f64 = 80.0;

const BG_CH: char = ' ';

const S_MAX: usize = (S_WIDTH * S_HEIGHT) as usize;

const SLEEP_DURATIOM: time::Duration = time::Duration::from_millis(50);

// const D_RESOLUTION: f64 = 1.0;

fn main() {
    let mut output_buffer: [char; S_MAX];
    let mut z_buffer: [f64; S_MAX];

    let mut transform = Quat::new(1.0, 0.0, 0.0, 0.0).normalize();

    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 0.0;

    println!("\x1b[2J");

    loop {
        output_buffer = [BG_CH; S_MAX];
        z_buffer = [0.0; S_MAX];
        // println!("{:?}", transform);

        let width = 15;
        // let offset = 10;

        for x in -width..width {
            for y in -width..width {
                let s0 = Quat::new(0.0, x as f64, y as f64, -width as f64);
                let s1 = Quat::new(0.0, width as f64, x as f64, y as f64);
                let s2 = Quat::new(0.0, -width as f64, y as f64, -x as f64);
                let s3 = Quat::new(0.0, -x as f64, y as f64, width as f64);
                let s4 = Quat::new(0.0, x as f64, -width as f64, y as f64);
                let s5 = Quat::new(0.0, x as f64, width as f64, y as f64);
                fill_buf(s0, &transform, &mut output_buffer, &mut z_buffer, '@');
                fill_buf(s1, &transform, &mut output_buffer, &mut z_buffer, '+');
                fill_buf(s2, &transform, &mut output_buffer, &mut z_buffer, '~');
                fill_buf(s3, &transform, &mut output_buffer, &mut z_buffer, ';');
                fill_buf(s4, &transform, &mut output_buffer, &mut z_buffer, '$');
                fill_buf(s5, &transform, &mut output_buffer, &mut z_buffer, '%');
            }
        }


        display(&output_buffer);
        // break;

        transform.increment(a, b, c);
        a += 0.1;
        b += 0.1;
        c += 0.1;

        thread::sleep(SLEEP_DURATIOM);
    }
}

fn fill_buf(point: Quat, transform: &Quat, buffer: &mut [char], z_buffer: &mut [f64], ch: char) {
    let point = point.tranform(*transform);

    let point = point.point();

    let z = point.z + OBJ_DIST;
    let ooz = 1.0/z;

    let xp = (S_WIDTH/2) + (S_DIST*ooz*point.x*2.0) as i32;
    let yp = (S_HEIGHT/2) + (S_DIST*ooz*point.y) as i32;

    let idx = (yp*S_WIDTH + xp) as usize;
    if idx < S_MAX {
        if ooz > z_buffer[idx] {
            z_buffer[idx] = ooz;
            buffer[idx] = ch;
        }
    }
}

fn display(display: &[char]) {
    println!("\x1b[2J");
    display.iter()
        .enumerate()
        .for_each(|(i, ch)| {
            print!("{}", ch);
            if i > 0 && i % S_WIDTH as usize == 0 {
                println!();
            }
        });

    stdout().flush().unwrap();
}

