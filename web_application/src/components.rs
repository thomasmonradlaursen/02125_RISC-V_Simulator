pub fn register(x_offset: f32, y_offset: f32, width: f32, height: f32) -> Vec<f32> {
    let (mut x,mut y) = draw_square(40.0, 300.0);
    translate_vertices(&mut x, &mut y, x_offset, y_offset);
    scale_to_canvas(x, y, width, height)
}

pub fn mem(x_offset: f32, y_offset: f32, width: f32, height: f32) -> Vec<f32> {
    let (mut x,mut y) = draw_square(80.0, 160.0);
    translate_vertices(&mut x, &mut y, x_offset, y_offset);
    scale_to_canvas(x, y, width, height)
}


pub fn pc(x_offset: f32, y_offset: f32, width: f32, height: f32) -> Vec<f32> {
    let (mut x,mut y) = draw_square(40.0, 80.0);
    translate_vertices(&mut x, &mut y, x_offset, y_offset);
    scale_to_canvas(x, y, width, height)
}

pub fn alu(x_offset: f32, y_offset: f32, width: f32, height: f32) -> Vec<f32> {
    let (mut x, mut y) = draw_adder();
    translate_vertices(&mut x, &mut y, x_offset, y_offset);
    scale_to_canvas(x, y, width, height)
}

pub fn multiplexer(x_offset: f32, y_offset: f32, width: f32, height: f32) -> Vec<f32> {
    let (mut x, mut y) = draw_multiplexer();
    translate_vertices(&mut x, &mut y, x_offset, y_offset);
    scale_to_canvas(x, y, width, height)
}

fn draw_square(width: f32, length: f32) -> (Vec<f32>, Vec<f32>) {
    let x: Vec<f32> = vec![0.0, 0.0,    0.0, width,     width, width,  width, 0.0];
    let y: Vec<f32> = vec![0.0, length,  length, length,  length, 0.0,  0.0, 0.0];
    (x,y)
}

fn draw_adder() -> (Vec<f32>, Vec<f32>) {
    let x: Vec<f32> = vec![0.0, 0.0,    0.0, 20.0,  20.0,   0.0,    0.0,    0.0, 0.0, 70.0, 70.0, 70.0, 70.0, 0.0];
    let y: Vec<f32> = vec![0.0, 60.0,  60.0, 80.0,  80.0, 100.0,  100.0,  160.0, 160.0, 120.0, 120.0, 40.0, 40.0, 0.0];
    (x,y)
}

fn draw_multiplexer() -> (Vec<f32>, Vec<f32>) {
    let x: Vec<f32> = vec![0.0, 0.0,    0.0, 30.0,  30.0, 30.0,  30.0,  0.0];
    let y: Vec<f32> = vec![0.0, 70.0,  70.0, 50.0,  50.0, 20.0,  20.0,  0.0];
    (x,y)
}


fn translate_vertices(x: &mut Vec<f32>, y: &mut Vec<f32>, x_offset: f32, y_offset: f32) {
    for x in x {
        *x += x_offset;
    }
    for y in y {
        *y += y_offset;
    }
}

fn scale_to_canvas(x: Vec<f32>, y: Vec<f32>, width: f32, height: f32) -> Vec<f32> {
    let ratio_width: f32 = width / 1160.0;
        let ratio_height: f32 = height / 600.0;
    let mut vertices: Vec<f32> = vec![];
    let zip = x.iter().zip(y.iter());
        for (x,y) in zip {
            let x = (x*ratio_width)/(width/2.0)-1.0;
            let y = (y*ratio_height)/(height/2.0)-1.0;
            vertices.push(x);
            vertices.push(y);

        }
    vertices
}