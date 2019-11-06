static mut BAN: [i8; 100] = [2; 100];

fn calc_pos(dir: i8, x: i8, y: i8) -> (i8, i8) {
   let next_x =
       if dir % 3 == 0 {
           x - 1
       } else if dir % 3 == 2 {
           x + 1
       } else {
           x
       };
   let next_y =
       if dir / 3 == 0 {
           y - 1
       } else if dir / 3 == 2 {
           y + 1
       } else {
           y
       };
   (next_x, next_y)
}

fn click_sub(turn: i8, cnt: i8, dir: i8, x: i8, y: i8) -> i8 {
   if x < 0 || y < 0 || x >= 10 || y >= 10 {
       return 0;
   }
   unsafe {
      if BAN[(y * 10 + x) as usize] == 2 {
          return 0;
      }
      if BAN[(y * 10 + x) as usize] == turn {
          return cnt;
      }
   }
   let (next_x, next_y) = calc_pos(dir, x, y);
   let res_cnt = click_sub(turn, cnt + 1, dir, next_x, next_y);
   if res_cnt != 0 {
      unsafe {
          BAN[(y * 10 + x) as usize] = turn;
      }
   }
   res_cnt
}

#[no_mangle]
pub fn click(turn: i8, x: i8, y: i8) -> i8 {
	unsafe {
	    if BAN[(y * 10 + x) as usize] != 2 {
	        return 0;
	    }
    }
    let mut cnt = 0;
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            let dir = i * 3 + j;
			let (next_x, next_y) = calc_pos(dir, x, y);
            cnt += click_sub(turn, 0, dir, next_x, next_y);
        }
    }
    if cnt != 0 {
        unsafe {
            BAN[(y * 10 + x) as usize] = turn
        }
        cnt + 1
    } else {
        0
    }
}

#[no_mangle]
pub fn get_array() -> *const i8 {
    unsafe {
        BAN.as_ptr()
    }
}

#[no_mangle]
pub fn init() {
    unsafe {
        BAN.iter_mut().for_each(|x| *x = 2);
        BAN[44] = 1;
        BAN[45] = 0;
        BAN[54] = 0;
        BAN[55] = 1;
    }
}
