use plotters::prelude::*;
use slint::SharedPixelBuffer;

pub fn render_plot(width: f32, height: f32) -> slint::Image {
    let mut pixel_buffer = SharedPixelBuffer::new(width as u32, height as u32);
    let size = (pixel_buffer.width(), pixel_buffer.height());
    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);

    let root = backend.into_drawing_area();
    root.fill(&WHITE).expect("error filling drawing area");
    let mut chart = ChartBuilder::on(&root)
        .margin(0)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-100f32..100f32, -2.0f32..2f32)
        .expect("error building coordinate system");
    chart.configure_mesh().draw().expect("error drawing");

    chart
        .draw_series(LineSeries::new(
            (-100..=100).map(|x| x as f32).map(|x| (x, x.sin())),
            &RED,
        ))
        .unwrap()
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x, y)], &RED));

    root.present().expect("error presenting");
    drop(chart);
    drop(root);
    slint::Image::from_rgb8(pixel_buffer)
}
