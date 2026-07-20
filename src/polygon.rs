use crate::framebuffer::Framebuffer;
use crate::line::Line;

pub trait Polygon {
    fn draw_polygon(
        &mut self,
        vertices: &[(i32, i32)],
        border: u32,
        fill: u32,
    );

    fn fill_polygon(
        &mut self,
        vertices: &[(i32, i32)],
        fill: u32,
    );

    fn draw_outline(
        &mut self,
        vertices: &[(i32, i32)],
        border: u32,
    );
}

impl Polygon for Framebuffer {
    fn draw_polygon(&mut self, vertices: &[(i32, i32)], border: u32, fill: u32,) {
        if vertices.len() < 3 {
            return;
        }
        
        self.fill_polygon(vertices, fill);
        self.set_current_color(border);

        for i in 0..vertices.len() {
            let (x1,y1)=vertices[i];
            let (x2,y2)=vertices[(i+1)%vertices.len()];

            self.line(
                x1 as usize,
                y1 as usize,
                x2 as usize,
                y2 as usize
            );
        }
    }

    fn fill_polygon(&mut self, vertices:&[(i32,i32)], fill:u32){
        if vertices.len() < 3 {
            return;
        }
        
        let ymin=vertices.iter().map(|v|v.1).min().unwrap();
        let ymax=vertices.iter().map(|v|v.1).max().unwrap();

        for y in ymin..=ymax{
            let mut nodes=Vec::<i32>::new();
            let mut j=vertices.len()-1;

            for i in 0..vertices.len(){
                let (xi,yi)=vertices[i];
                let (xj,yj)=vertices[j];

                if yi != yj && ((yi < y && yj >= y) || (yj < y && yi >= y)) {
                    let x = xi + (y - yi) * (xj - xi) / (yj - yi);
                    nodes.push(x);
                }

                j=i;
            }

            nodes.sort_unstable();

            let mut k=0;

            while k+1<nodes.len(){
                for x in nodes[k]..nodes[k+1]{
                    self.point_color(
                        x as usize,
                        y as usize,
                        fill
                    );
                }
                k+=2;
            }
        }
    }

    fn draw_outline(&mut self, vertices: &[(i32, i32)], border: u32) {
        if vertices.len() < 2 {
            return;
        }

        self.set_current_color(border);

        for i in 0..vertices.len() {
            let (x1, y1) = vertices[i];
            let (x2, y2) = vertices[(i + 1) % vertices.len()];

            self.line(
                x1 as usize,
                y1 as usize,
                x2 as usize,
                y2 as usize,
            );
        }
    }
}