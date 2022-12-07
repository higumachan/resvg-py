use numpy::ndarray::Array3;
use numpy::{IntoPyArray, PyArray3};
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn render_svg<'py>(py: Python<'py>, svg_xml: &str, zoom_ratio: f32) -> PyResult<&'py PyArray3<u8>> {
    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_str(svg_xml, &opt.to_ref()).unwrap();
    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(
        (pixmap_size.width() as f32 * zoom_ratio).ceil() as u32,
        (pixmap_size.height() as f32 * zoom_ratio).ceil() as u32,
    )
    .unwrap();

    resvg::render(
        &rtree,
        usvg::FitTo::Zoom(zoom_ratio),
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();

    let pixels = pixmap.pixels();

    Ok(Array3::from_shape_fn(
        ((pixmap.height()) as usize, (pixmap.width()) as usize, 4),
        |(y, x, c)| {
            let index = y * pixmap.width() as usize + x;
            let pixel = &pixels[index];

            match c {
                0 => pixel.red(),
                1 => pixel.green(),
                2 => pixel.blue(),
                3 => pixel.alpha(),
                _ => unreachable!(),
            }
        },
    )
    .into_pyarray(py))
}

/// A Python module implemented in Rust.
#[pymodule]
fn resvg_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render_svg, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
