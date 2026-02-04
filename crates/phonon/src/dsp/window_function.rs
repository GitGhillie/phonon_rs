use std::f32::consts::PI;

/// Produces a tukey window of size `size` + `overlap_size`
pub(crate) fn tukey(size: usize, overlap_size: usize) -> Vec<f32> {
    let k = overlap_size - 1;
    let n = size + overlap_size;

    let mut window: Vec<f32> = vec![1.0; n];

    for i in 0..overlap_size {
        window[i] = 0.5 * (1.0 + (PI * ((i as f32) / (k as f32) - 1.0)).cos())
    }

    for i in size..n {
        window[i] = 0.5 * (1.0 + (PI * (1.0 - ((n - i - 1) as f32) / (k as f32))).cos())
    }

    window
}

#[cfg(test)]
mod tests {
    use super::*;
    use plotters::prelude::*;

    #[ignore = "visual check only."]
    #[test]
    fn test_tukey_visual() -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("figures/tukey_window.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("tukey window", ("sans", 30).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-0.1f32..200f32, -0.1f32..1.1f32)?;

        chart.configure_mesh().draw()?;

        let size = 100;
        let overlap_size = 30;
        let window = tukey(size, overlap_size);

        chart
            .draw_series(LineSeries::new(
                window.iter().enumerate().map(|(a, b)| (a as f32, *b)),
                &RED,
            ))?
            .label("input")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

        Ok(())
    }
}
