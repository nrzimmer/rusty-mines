use rand::Rng;

pub struct PlayField {
    width: usize,
    height: usize,
    open_vec: Vec<bool>,
    bomb_vec: Vec<bool>,
    flag_vec: Vec<bool>,
    game_over: bool,
}

pub type GameStateResult = Result<GameState, String>;

#[derive(Copy, Clone)]
pub enum PlayFieldSpot {
    Closed,
    Flagged,
    Bomb,
    Open(u8),
}

#[derive(Copy, Clone)]
pub enum GameState {
    KeepPlaying,
    YouWin,
    YouLose,
}

impl PlayField {
    pub fn new(w: usize, h: usize, bombs: usize) -> PlayField {
        let size = h * w;
        let mut b = vec![false; size];

        let mut rng = rand::thread_rng();

        let mut count = 0;
        while count < bombs {
            let pos = rng.gen_range(0..size);
            if !b[pos] {
                b[pos] = true;
                count += 1;
            }
        }

        let o = vec![false; size];
        let f = vec![false; size];

        PlayField {
            width: w,
            height: h,
            open_vec: o,
            flag_vec: f,
            bomb_vec: b,
            game_over: false,
        }
    }

    fn index_from_coord(&self, x: usize, y: usize) -> Result<usize, String> {
        if x > self.width || x == 0 {
            return Err("Invalid X coordinate".to_string());
        }
        if y > self.height || y == 0 {
            return Err("Invalid Y coordinate".to_string());
        }
        Ok((y - 1) * self.width + (x - 1))
    }

    fn coord_from_index(&self, index: usize) -> Result<(usize, usize), String> {
        if index > self.bomb_vec.len() {
            return Err("Invalid index".to_string());
        }
        Ok((index % self.width + 1, index / self.width + 1))
    }

    fn get_valid_neighbors(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::with_capacity(8);
        v.push((x - 1, y - 1));
        v.push((x, y - 1));
        v.push((x + 1, y - 1));
        v.push((x - 1, y));
        v.push((x + 1, y));
        v.push((x - 1, y + 1));
        v.push((x, y + 1));
        v.push((x + 1, y + 1));
        v.retain(|&(xx, yy)| match self.index_from_coord(xx, yy) {
            Err(_) => false,
            Ok(_) => true,
        });
        v
    }

    fn inner_open(&mut self, x: usize, y: usize) {
        let pos = match self.index_from_coord(x, y) {
            Err(_) => return,
            Ok(v) => v,
        };

        if self.bomb_vec[pos] || self.flag_vec[pos] {
            return;
        }

        self.open_vec[pos] = true;

        if self.get_bombs_around(pos) == 0 {
            for (xx, yy) in self.get_valid_neighbors(x, y) {
                let pos = match self.index_from_coord(xx, yy) {
                    Err(_) => continue,
                    Ok(v) => v,
                };
                if !self.open_vec[pos] {
                    self.inner_open(xx, yy);
                }
            }
        }
    }

    pub fn open(&mut self, x: usize, y: usize) -> GameStateResult {
        let pos = match self.index_from_coord(x, y) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        if self.flag_vec[pos] {
            return Err("Cannot open flagged coordinate".to_string());
        }

        if self.bomb_vec[pos] {
            self.game_over = true;
            return Ok(GameState::YouLose);
        }

        if self.open_vec[pos] {
            return Err("Cannot open already open coordinate".to_string());
        }

        self.inner_open(x, y);

        Ok(GameState::KeepPlaying)
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) -> GameStateResult {
        let index = match self.index_from_coord(x, y) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        if self.open_vec[index] {
            return Err("Cannot tag open coordinate".to_string());
        }

        self.flag_vec[index] = true;

        if self.bomb_vec == self.flag_vec {
            self.game_over = true;
            return Ok(GameState::YouWin);
        }

        Ok(GameState::KeepPlaying)
    }

    fn get_bombs_around(&mut self, index: usize) -> u8 {
        let (x, y) = match self.coord_from_index(index) {
            Err(_) => return 0,
            Ok(w) => w,
        };
        let v = self.get_valid_neighbors(x, y);
        let mut bombs = 0;
        for (x, y) in v {
            let pos = match self.index_from_coord(x, y) {
                Err(_) => continue,
                Ok(w) => w,
            };

            if self.bomb_vec[pos] {
                bombs += 1;
            }
        }
        bombs
    }

    pub fn get_playfield(&mut self) -> Vec<PlayFieldSpot> {
        let mut p = Vec::with_capacity(self.open_vec.len());
        for index in 0..self.open_vec.len() {
            if self.open_vec[index] {
                p.push(PlayFieldSpot::Open(self.get_bombs_around(index)));
            } else if self.flag_vec[index] {
                p.push(PlayFieldSpot::Flagged);
            } else if self.game_over && self.bomb_vec[index] {
                p.push(PlayFieldSpot::Bomb);
            } else {
                p.push(PlayFieldSpot::Closed);
            }
        }
        p
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}
