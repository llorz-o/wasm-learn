use cfg_if::cfg_if;
use fixedbitset::FixedBitSet;
use js_sys;
#[allow(unused_imports)]
use log::*;
use wasm_bindgen::prelude::*;
use web_sys;

mod utils;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log(){}
    }
}

pub struct Time<'a> {
    name: &'a str,
}

impl<'a> Time<'a> {
    pub fn new(name: &'a str) -> Self {
        web_sys::console::time_with_label(name);
        Time { name }
    }
}

impl<'a> Drop for Time<'a> {
    fn drop(&mut self) {
        web_sys::console::time_end_with_label(self.name);
    }
}

// 导入js函数
// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// 导出rs函数
// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, wasm-game-of-life!");
// }

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
    init_log();
}

// #[wasm_bindgen]
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Cell {
//     Dead = 0,
//     Alive = 1,
// }

/*
   cells 是个数组 数组的每个段落为一个 row
   [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]
   [
    [ 0, 1, 2, 3], row:0 * width:4 + 0,1,2,3
    [ 4, 5, 6, 7], row:1 * width:4 + 0,1,2,3
    [ 8, 9,10,11], row:2 * width:4 + 0,1,2,3
    [12,13,14,15], row:3 * width:4 + 0,1,2,3
   ]

   在一个有限的内存中要形成一个无限的空间,需要使对立边界相互关联
   在如上所示的 4*4 的空间内,取坐标 x4,y4
   最终坐标为
   [0,0]
   当前坐标为 (x,y)
   最终坐标为 (x%w,y%h) -> (0,0)
   对于无限边界(x,y) 的上下左右进行偏移
   (x-1,y-1) (x,y-1) (x+1,y-1)
   (x-1,y)   (x,y)   (x+1,y)
   (x-1,y+1) (x,y+1) (x+1,y+1)

   以上所有 (x|y) - 1 均可表示为 (x|y) + (-1)

    -1 (-1,-1) (0,-1) (1,-1)
     0 (-1,0)  (0,0)  (1,0)
     1 (-1,1)  (0,1)  (1,1)
     y
     x  -1      0      1

   可表示为 (x,y); x in [-1,0,1], y in[-1,0,1]

   对于有限边界(w,h)
   (x-1,y-1) 可表示为
   (w-1,h-1) => (3,3)

   所以对于任意坐标(a,b)的(-1,-1) 的无限坐标位置为
   ((a + w-1)%w,(b + h-1)%h) ->

*/

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

// impl fmt::Display for Universe {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // 转换为切片,按宽度分块
//         for line in self.cells.as_slice().chunks(self.width as usize) {
//             for &cell in line {
//                 let symbol = if cell { "□" } else { "■" };
//                 write!(f, "{}", symbol)?;
//             }
//             write!(f, "\n")?;
//         }
//         Ok(())
//     }
// }

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
    // 计算相邻单位的存活状态
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        /*
            内存消耗过大,下面是优化代码
            let mut count = 0;

            let north = if row == 0 {
                self.height - 1
            } else {
                row - 1
            };

            let south = if row == self.height - 1 {
                0
            } else {
                row + 1
            };

            let west = if column == 0 {
                self.width - 1
            } else {
                column - 1
            };

            let east = if column == self.width - 1 {
                0
            } else {
                column + 1
            };

            let nw = self.get_index(north, west);
            count += self.cells[nw] as u8;

            let n = self.get_index(north, column);
            count += self.cells[n] as u8;

            let ne = self.get_index(north, east);
            count += self.cells[ne] as u8;

            let w = self.get_index(row, west);
            count += self.cells[w] as u8;

            let e = self.get_index(row, east);
            count += self.cells[e] as u8;

            let sw = self.get_index(south, west);
            count += self.cells[sw] as u8;

            let s = self.get_index(south, column);
            count += self.cells[s] as u8;

            let se = self.get_index(south, east);
            count += self.cells[se] as u8;

            count
        */
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8
            }
        }
        count
    }
    pub fn trick(&mut self) {
        let timer = Time::new("trick begin");
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbor_count = self.live_neighbor_count(row, col);

                // panic!("test");

                // log!(
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors", row, col, cell, live_neighbor_count
                // );

                let next_cell = match (cell, live_neighbor_count) {
                    // 任何少于两个活邻居的活细胞死亡，好像是由人口不足造成的。
                    (true, x) if x < 2 => false,
                    // 任何有两个或三个活邻居的活细胞都住着下一代。
                    (true, 2) | (true, 3) => true,
                    // 任何有超过三个活邻居的活细胞死亡，就像人口过剩。
                    (true, x) if x > 3 => false,
                    // 任何有三个活邻居的死细胞都变成了活细胞，就像通过繁殖一样。
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };
                next.set(idx, next_cell);
            }
        }

        self.cells = next;
    }
    pub fn new() -> Universe {
        let width = 120;
        let height = 120;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            let random = js_sys::Math::random();
            cells.set(i, random > 0.5);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    // pub fn render(&self) -> String {
    //     self.to_string()
    // }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn set_width_height(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, false);
        }
        self.cells = cells;
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.toggle(idx);
        self.output()
    }

    pub fn dead_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.set(idx, true);
    }

    pub fn dead_universe(&mut self) {
        for i in 0..(self.width * self.height) as usize {
            self.cells.set(i, false)
        }
    }

    pub fn reset_universe(&mut self) {
        for i in 0..(self.width * self.height) as usize {
            self.cells.set(i, js_sys::Math::random() > 0.5)
        }
    }

    fn output(&self) {
        let mut str = String::from("");
        for i in self.cells.as_slice() {
            str = format!("{} {}", str, i);
        }
        log!("u32: {}", str);
    }
}

// 这里无法使用 wasm 导出,因为js无法支持元组特性
impl Universe {
    pub fn get_cells(&self) -> &[u32] {
        &self.cells.as_slice()
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

