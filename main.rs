
use nannou::prelude::*;



#[derive(Copy, Clone)]
struct Plot {
    x: f32,
    y: f32,
}


struct Model {
    land: Vec<Plot>,
    window_id: window::Id,
}

fn generate_land_plot() -> Plot {
    let random_x_range = random_range(-500.0, 500.0);
    let random_y_range = random_range(200.0, 500.0);

    Plot {x: random_x_range, y: random_y_range }


}


fn init(app: &App) -> Model {
    //window builder to call for resources to create a window.
    let window_id = app
    .new_window()
    .view(view)                        
    .build()
    .unwrap(); //crashes if error with creating window.

    //initialize a Vector of Plots. Each index will create a new plot on the canvas.
    let mut land = vec![];
    for _ in 0..1001 {
        land.push(generate_land_plot());
    }
    




    Model {
        land,
        window_id,
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let random_y_len: f32 = random_range(14.0, 26.0);

    

    
    let draw = app.draw(); //initialize something I can draw onto. Canvas.

    draw
    .background()
    .color(PLUM); //background color of canvas I am drawing on



    //iterate through each plot in Model { land } vector and
    //draw plots on canvas. w_h() controls the plot size on canvas
    //x() and y() coordinate with location of plot.
    //color() changes plot color. 
    for plot in model.land.iter() {
        draw
        .rect()
        .w_h(2.0, random_y_len)
        .x(plot.x)
        .y(plot.y)
        .color(ORCHID);
    }

    draw
    .to_frame(app, &frame) //actually call the renderer to screen and show canvas in window.
    .unwrap() //crashes if renderer cant be created

}

fn update(app: &App, model: &mut Model, _update: Update) {

    for plot in model.land.iter_mut() {
        let random_offset: f32 = random_range(-500.0, 500.0);
        let random_down_speed: f32 = random_range(0.1, 51.0);
        if plot.x >= 0.0 || plot.x <= 0.0 || plot.x == 0.0 {
            plot.x = plot.x + random_offset;
            if plot.x >= 1000.0 {
                plot.x = 0.0 + random_offset;
            }
        }

        plot.y = plot.y - random_down_speed;

        if plot.y <= 200.0 {
            plot.y = plot.y - random_down_speed + 15.0;
        } else {
            plot.y = plot.y - random_down_speed + 20.0;
        }


        if plot.y <= -500.0 {
            plot.y = 500.0;
            
        }
       
    }

   
    //draw the update to canvas
    app.draw();
}

fn main() {


    nannou::app(init).update(update).run();
    

}
