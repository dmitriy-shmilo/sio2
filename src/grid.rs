use crate::{
    util::wrap,  FIELD_HEIGHT, FIELD_HEIGHT_F32, FIELD_WIDTH,
    FIELD_WIDTH_F32, TEXTURE_STRIDE, TEXTURE_TYPE,
};

use bevy::prelude::*;
use bevy::render::texture::Texture;
pub struct Grid {
    particles: Vec<Option<Entity>>,
    pub xsize: usize,
    pub ysize: usize,
    pub background_color: [u8; 4],
    pub texture: Texture,
}

fn color_to_bytes(color: Color) -> [u8; 4] {
    [
        (color.r() * 255.99) as u8,
        (color.g() * 255.99) as u8,
        (color.b() * 255.99) as u8,
        (color.a() * 255.99) as u8,
    ]
}

impl Default for Grid {
    fn default() -> Self {
        let bg = color_to_bytes(Color::rgb(0.11, 0.11, 0.11));
        Self {
            particles: { vec![None; FIELD_WIDTH * FIELD_HEIGHT] },
            xsize: FIELD_WIDTH,
            ysize: FIELD_HEIGHT,
            background_color: bg,
            texture: Texture::new_fill(
                Vec2::new(FIELD_WIDTH_F32, FIELD_HEIGHT_F32),
                &bg,
                TEXTURE_TYPE,
            ),
        }
    }
}

impl Grid {
    pub fn wrap_xy(&self, x: i32, y: i32) -> (usize, usize) {
        (
            wrap(x, 0, self.xsize as i32) as usize,
            wrap(y, 0, self.ysize as i32) as usize,
        )
    }
    pub fn grid_index(&self , idx: (usize , usize)) -> usize{
        let (x , y) = idx;
        y * self.ysize + x
    }
    pub fn texture_offset(&self , idx: (usize , usize)) -> usize{
        let (x , y) = idx;
        (x + (self.ysize - y - 1) * self.xsize) * TEXTURE_STRIDE
    }
    pub fn get(&self , x: i32 , y: i32) -> Option<Entity>{
        self.particles[self.grid_index(self.wrap_xy(x , y))]
    }
    pub fn copy_into_texture(&mut self , idx: (usize , usize) , src: [u8;4]){
        let offset = self.texture_offset(idx);
        self.texture.data[offset..(offset + 4)].copy_from_slice(&src);
    }
    pub fn set(&mut self , x:  i32 , y: i32 , e: Entity , color: Color){
        let idx = self.wrap_xy(x , y);
        self.copy_into_texture(idx , color_to_bytes(color));
        let gidx = self.grid_index(idx);
        self.particles[gidx] = Some(e);
    }
    pub fn remove(&mut self , x:i32 ,y:i32) -> Option<Entity>{
        let idx = self.wrap_xy(x , y);
        self.copy_into_texture(idx , self.background_color);
        let gidx = self.grid_index(idx);
        self.particles[gidx].take()
    }

    pub fn swap(&mut self, x1: i32 , y1: i32 , x2: i32 , y2: i32){
        let (idx1 , idx2) = (self.wrap_xy(x1 , y1) , self.wrap_xy(x2 , y2));
        let (gidx1 , gidx2) = (self.grid_index(idx1) , self.grid_index(idx2));
        self.particles.swap(gidx1 , gidx2);
        let (off1 , off2) = (self.texture_offset(idx1) , self.texture_offset(idx2));
        self.texture.data.swap(off1 , off2);
        self.texture.data.swap(off1 + 1, off2 + 1);
        self.texture.data.swap(off1 + 2, off2 + 2);
        self.texture.data.swap(off1 + 3, off2 + 3);
    }
}

/*

pub struct Grid {
    particles: FxHashMap<(i32, i32), Entity>,
    texture: Texture,
    pub xsize: usize,
    pub ysize: usize,
    pub background_color: [u8; 4],
}

fn color_to_bytes(color: Color) -> [u8; 4] {
    [
        (color.r() * 255.99) as u8,
        (color.g() * 255.99) as u8,
        (color.b() * 255.99) as u8,
        (color.a() * 255.99) as u8,
    ]
}

impl Default for Grid {
    fn default() -> Self {
        let bg = color_to_bytes(Color::rgb(0.11, 0.11, 0.11));
        Self {
            particles: {
                let mut fmap = FxHashMap::default();
                fmap.reserve(
                    ((FIELD_WIDTH_F32 * FIELD_HEIGHT_F32) * DEFAULT_HMAP_SIZE_PERCENT) as usize,
                );
                fmap
            },
            xsize: FIELD_WIDTH,
            ysize: FIELD_HEIGHT,
            background_color: bg,
            texture: Texture::new_fill(
                Vec2::new(FIELD_WIDTH_F32, FIELD_HEIGHT_F32),
                &bg,
                TEXTURE_TYPE,
            ),
        }
    }
}

impl Grid {
    /*
    pub fn iter_entitys(&self) -> impl Iterator<Item = (&(i32, i32), &Entity)> + '_ {
        self.particles.iter().map(|x| (x.0, &x.1 .0))
    }
    pub fn iter(&self) -> impl Iterator<Item = (&(i32, i32), &(Entity, [u8; 4]))> + '_ {
        self.particles.iter()
    }
    */
    fn wrap_x(&self , x: i32) -> i32{
        wrap(x , 0 , self.xsize as i32)
    }
    fn wrap_y(&self, y: i32) -> i32{
        wrap(y , 0 , self.ysize as i32)
    }
    fn wrap_indexs(&self, x: i32, y: i32) -> (i32, i32) {
        (x, y)
    }
    fn texture_index(&self , x: i32 , y: i32) -> usize{
        (x as usize + (self.ysize - y as usize - 1) * self.xsize) * TEXTURE_STRIDE
    }
    fn copy_into_texture(&mut self , offset: usize, data: [u8;4]){
        self.texture.data[offset..(offset + TEXTURE_STRIDE)].copy_from_slice(&data);
    }
    pub fn get_non_wrapping(&self, x: i32, y: i32) -> Option<Entity> {
        self.particles.get(&(x, y)).copied()
    }
    pub fn get(&self, x: i32, y: i32) -> Option<Entity> {
        let (x, y) = self.wrap_indexs(x, y);
        self.get_non_wrapping(x, y)
    }
    pub fn set(&mut self, x: i32, y: i32, value: (Entity, [u8; 4])) {
        let (xw , yw) = self.wrap_indexs(x, y);
        self.particles.insert((xw , yw), value.0);
        let ti = self.texture_index(xw , yw);
        self.copy_into_texture(ti , value.1);
    }
    // non pub as invalid index will cause panic on rendering texture
    fn set_non_wrapping(&mut self, x: i32, y: i32, value: (Entity, [u8; 4])) {
        self.particles.insert((x, y), value.0);
        self.copy_into_texture(self.texture_index(x , y) , value.1);
    }
    pub fn remove(&mut self, x: i32, y: i32) -> Option<Entity> {
        let (xw , yw) = self.wrap_indexs(x , y);
        self.copy_into_texture(self.texture_index(xw , yw) , self.background_color);
        self.particles.remove(&(xw , yw))
    }
    pub fn swap(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        if (x1, y1) == (x2, y2) {
            return;
        }
        let (idx1, idx2) = (self.wrap_indexs(x1, y1), self.wrap_indexs(x2, y2));
        let (txi1, txi2) = (self.texture_index(idx1.0 , idx2.1) , self.texture_index(idx2.0 , idx2.1));
        let e1 = self.particles.get(&idx1).copied();
        let e2 = self.particles.get(&idx2).copied();
        if let Some(e) = e1 {
            self.set_non_wrapping(idx2.0, idx2.1, e);
        } else {
            self.remove_non_wrapping(idx2.0, idx2.1);
        }
        if let Some(e) = e2 {
            self.set_non_wrapping(idx1.0, idx1.1, e);
        } else {
            self.remove_non_wrapping(idx1.0, idx1.1);
        }
    }
}
*/
