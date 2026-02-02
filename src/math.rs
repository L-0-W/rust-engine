pub fn normalize<T: Into<f64>>(pos_x: f64, pos_y: f64, width: T, height: T) -> (f64, f64) {
   let normalize_x = 2.0 * (pos_x - 0.0) / (width.into() - 0.0) - 1.0;
   let normalize_y = 2.0 * (pos_y - 0.0) / (height.into() - 0.0)  - 1.0;

   return (normalize_x, normalize_y)
}
