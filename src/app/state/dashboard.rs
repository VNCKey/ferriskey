pub struct DashboardState {
    pub show_ingresos: bool,
    pub show_gastos: bool,
    pub show_beneficios: bool,
    pub year: i32,
    pub dash_tab: usize,
    pub bcr_playing: bool,
    pub bcr_year: f32,
    pub bcr_speed: f32,
    pub pie_donut_hole: f32,
    pub pie_exploded: bool,
    pub index_baseline_year: f32,
    pub ts_show_ma: bool,
    pub ts_show_volume: bool,
    pub ts_show_rsi: bool,
    pub bool_sim_a: bool,
    pub bool_sim_b: bool,
}

impl Default for DashboardState {
    fn default() -> Self {
        Self {
            show_ingresos: true,
            show_gastos: true,
            show_beneficios: true,
            year: 2025,
            dash_tab: 0,
            bcr_playing: false,
            bcr_year: 2015.0,
            bcr_speed: 1.0,
            pie_donut_hole: 0.5,
            pie_exploded: false,
            index_baseline_year: 2020.0,
            ts_show_ma: true,
            ts_show_volume: true,
            ts_show_rsi: true,
            bool_sim_a: true,
            bool_sim_b: false,
        }
    }
}
