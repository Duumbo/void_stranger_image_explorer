use raylib::{ffi::{GetScreenHeight, GetScreenWidth, SetConfigFlags}, prelude::*};
use std::fs;

const WIDTH: i32 = 3840;
const HEIGHT: i32 = 2160;
const CURSOR_WIDTH: f32 = 5.0;

struct Room {
    texture: Texture2D,
    posx: i32,
    posy: i32,
}

struct Point {
    x: i32,
    y: i32,
}

fn load_room_textures(rl: &mut RaylibHandle, thread: &RaylibThread) -> Vec<Room> {
    let mut rooms: Vec<Room> = Vec::new();
    let floors = fs::read_dir("floors/").unwrap();
    let mut nx = 0;
    let mut ny = 0;
    for floor_dir_entry in floors.into_iter() {
        let floor_path = floor_dir_entry.unwrap().path();
        let floor_str = floor_path.to_str().unwrap();
        let mut floor = Image::load_image(floor_str).unwrap();
        floor.resize(WIDTH, HEIGHT);
        let floor_tex = rl.load_texture_from_image(thread, &floor).unwrap();
        rooms.push(Room { texture: floor_tex, posx: nx, posy: ny });
        nx += 1;
        if nx >= 6 {
            nx = 0;
            ny += 1;
        }
    }
    rooms
}

fn update_selection(r: &mut Rectangle, x: i32, y: i32, w: i32, h: i32) {
    println!("at ({}, {})", x, y);
    r.x = x as f32;
    r.y = y as f32;
    r.width = w as f32;
    r.height = h as f32;
}

fn keyboard_events(d: &mut RaylibHandle, cursor1: &mut Point, cursor2: &mut Point, key_flags: &mut [bool]) {
    // Cursor 1
    if d.is_key_up(KeyboardKey::KEY_J) && key_flags[0] {
        key_flags[0] = false;
    }
    if d.is_key_down(KeyboardKey::KEY_J) && !key_flags[0] {
        cursor1.y += 1;
        key_flags[0] = true;
    }
    if d.is_key_up(KeyboardKey::KEY_K) && key_flags[1] {
        key_flags[1] = false;
    }
    if d.is_key_down(KeyboardKey::KEY_K) && !key_flags[1] {
        cursor1.y -= 1;
        key_flags[1] = true;
    }
    if d.is_key_up(KeyboardKey::KEY_H) && key_flags[2] {
        key_flags[2] = false;
    }
    if d.is_key_down(KeyboardKey::KEY_H) && !key_flags[2]{
        cursor1.x -= 1;
        key_flags[2] = true;
    }
    if d.is_key_up(KeyboardKey::KEY_L) && key_flags[3] {
        key_flags[3] = false;
    }
    if d.is_key_down(KeyboardKey::KEY_L) && ! key_flags[3]{
        cursor1.x += 1;
        key_flags[3] = true;
    }
    // Cursor 2
    if d.is_key_up(KeyboardKey::KEY_D) && key_flags[4] {
        key_flags[4] = false;
    }
    if d.is_key_down(KeyboardKey::KEY_D) && !key_flags[4] {
        cursor2.y += 1;
        key_flags[4] = true;
    }
    if d.is_key_up(KeyboardKey::KEY_F) && key_flags[5] {
        key_flags[5] = false;
    }
    if d.is_key_down(KeyboardKey::KEY_F) && !key_flags[5] {
        cursor2.y -= 1;
        key_flags[5] = true;
    }
    if d.is_key_up(KeyboardKey::KEY_A) && key_flags[6] {
        key_flags[6] = false;
    }
    if d.is_key_down(KeyboardKey::KEY_A) && !key_flags[6]{
        cursor2.x -= 1;
        key_flags[6] = true;
    }
    if d.is_key_up(KeyboardKey::KEY_G) && key_flags[7] {
        key_flags[7] = false;
    }
    if d.is_key_down(KeyboardKey::KEY_G) && ! key_flags[7]{
        cursor2.x += 1;
        key_flags[7] = true;
    }
}

fn draw_selection(d: &mut RaylibDrawHandle, cursor: &Rectangle, color: Color) {
    let left = Rectangle {
     x: cursor.x,
     y: cursor.y,
     width: CURSOR_WIDTH,
     height: cursor.height
    };
    d.draw_rectangle_rec(left, color);
    let right = Rectangle {
     x: cursor.x + cursor.width - CURSOR_WIDTH,
     y: cursor.y,
     width: CURSOR_WIDTH,
     height: cursor.height
    };
    d.draw_rectangle_rec(right, color);
    let top = Rectangle {
     x: cursor.x,
     y: cursor.y,
     width: cursor.width,
     height: CURSOR_WIDTH
    };
    d.draw_rectangle_rec(top, color);
    let bottom = Rectangle {
     x: cursor.x,
     y: cursor.y + cursor.height - CURSOR_WIDTH,
     width: cursor.width,
     height: CURSOR_WIDTH
    };
    d.draw_rectangle_rec(bottom, color);
}

fn main() {
    unsafe {SetConfigFlags(4)}
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Hello, World")
        .build();
    let mut rooms = load_room_textures(&mut rl, &thread);
    let nfloors = rooms.len();
    let tex_rec = Rectangle {
        x: 0.0,
        y: 0.0,
        width: WIDTH as f32,
        height: HEIGHT as f32,
    };
    let pos = Vector2 {
        x: 0.0,
        y: 0.0,
    };
    let mut cursor_rec1 = Rectangle {
        x: 0.0,
        y: 0.0,
        width: WIDTH as f32,
        height: HEIGHT as f32,
    };
    let mut cursor_coords1 = Point {
        x: 0,
        y: 0,
    };
    let mut cursor_rec2 = Rectangle {
        x: 0.0,
        y: 0.0,
        width: WIDTH as f32,
        height: HEIGHT as f32,
    };
    let mut cursor_coords2 = Point {
        x: 0,
        y: 0,
    };
    let mut key_flags = vec![false; 9];


    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let width = unsafe {
            GetScreenWidth()
        };
        let height = unsafe {
            GetScreenHeight()
        };
        if d.is_key_down(KeyboardKey::KEY_ENTER) {
            let mut output = Image::gen_image_color(WIDTH, HEIGHT, Color::WHITE);
            for i in 0..nfloors {
                let floor_rec = Rectangle {
                    x: (rooms[i].posx * WIDTH / 6 ) as f32,
                    y: (rooms[i].posy * HEIGHT / 6 ) as f32,
                    width: (WIDTH / 6 ) as f32,
                    height: (HEIGHT / 6 ) as f32,
                };
                let mut floor_image = rooms[i].texture.load_image().unwrap();
                floor_image.resize(WIDTH/6, HEIGHT/6);
                output.draw(&floor_image, tex_rec, floor_rec, Color::WHITE);
            }

            for x in 0..6 {
                output.draw_line(x * WIDTH/6, 0, x * WIDTH/6, HEIGHT, Color::BLACK);
            }
            for y in 0..6 {
                output.draw_line(0, y * HEIGHT/6, WIDTH, y*HEIGHT/6, Color::BLACK);
            }
            output.export_image("win.png");
            break;
        }
        keyboard_events(&mut d, &mut cursor_coords1, &mut cursor_coords2, &mut key_flags);
        update_selection(&mut cursor_rec1, cursor_coords1.x * width / 6, cursor_coords1.y * height/6, width/6, height/6);
        update_selection(&mut cursor_rec2, cursor_coords2.x * width / 6, cursor_coords2.y * height/6, width/6, height/6);

        if key_flags[8] && d.is_key_up(KeyboardKey::KEY_S) {
            key_flags[8] = false;
        }
        if d.is_key_down(KeyboardKey::KEY_S) && !key_flags[8] {
            key_flags[8] = true;
            for i in 0..nfloors {
                let x = rooms[i].posx;
                let y = rooms[i].posy;
                if x == cursor_coords1.x && y == cursor_coords1.y {
                    rooms[i].posx = cursor_coords2.x;
                    rooms[i].posy = cursor_coords2.y;
                }
                if x == cursor_coords2.x && y == cursor_coords2.y {
                    rooms[i].posx = cursor_coords1.x;
                    rooms[i].posy = cursor_coords1.y;
                }
            }
        }

        d.clear_background(Color::WHITE);
        for i in 0..nfloors {
            let floor_rec = Rectangle {
                x: (rooms[i].posx * width / 6 ) as f32,
                y: (rooms[i].posy * height / 6 ) as f32,
                width: (width / 6 ) as f32,
                height: (height / 6 ) as f32,
            };
            d.draw_texture_pro(&rooms[i].texture, tex_rec, floor_rec, pos, 0.0, Color::WHITE);
        }

        for x in 0..6 {
            d.draw_line(x * width/6, 0, x * width/6, height, Color::BLACK);
        }
        for y in 0..6 {
            d.draw_line(0, y * height/6, width, y*height/6, Color::BLACK);
        }

        draw_selection(&mut d, &cursor_rec1, Color::BLUE);
        draw_selection(&mut d, &cursor_rec2, Color::RED);
        //d.draw_rectangle_rec(selection_rec, Color::BLUE);
    }
}
