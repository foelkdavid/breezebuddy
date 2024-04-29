use crate::actions;
use gtk::{prelude::*, Application, ApplicationWindow, Button, Grid, Label};
use gtk4 as gtk;

/// builds a sensor widget
fn create_sensor_widget(label: &String, temp: f32) -> Label {
    let widget_content: &str = &format!("{}: \n{}°C", label, temp);
    let widget = Label::new(Some(widget_content));
    widget.set_width_request(100);
    widget.set_widget_name("sensor_widget");
    widget.set_halign(gtk::Align::Start);
    return widget;
}

/// fills a grid with sensor widgets
fn fill_sensor_grid(grid: Grid) {
    let sensors = actions::get_sensors();
    for (index, (label, &temp)) in sensors.iter().enumerate() {
        let sensor_widget = create_sensor_widget(label, temp);

        // calculate columns
        let attach_row = if index >= 3 { 2 } else { 1 };
        let attach_column = (index % 3 + 1) as i32;

        grid.attach(&sensor_widget, attach_column, attach_row, 1, 1);
    }
}

/// fills a grid with curve widgets
///
/// -- currently only dummy content!!
fn fill_curve_grid(grid: Grid) {
    for index in 1..=4 {
        let curve = Label::new(Some(&format!("Curve {}", index)));
        curve.set_widget_name("curve_widget");
        curve.set_height_request(250);
        curve.set_width_request(450);

        let attach_row = (index + 1) / 2 + 5; // Calculate the row for attachment
        let attach_column = index % 2 + 1; // Calculate the column for attachment

        grid.attach(&curve, attach_column, attach_row, 1, 1);
    }
}

/// fills the left menu part of the UI
fn fill_left_grid(left_grid: &Grid) {
    let main_heading = Label::new(Some("❄️ BreezeBuddy"));
    main_heading.set_widget_name("main_heading");
    main_heading.set_margin_top(20);
    main_heading.set_margin_bottom(100);
    left_grid.attach(&main_heading, 1, 1, 1, 1);

    let button_options = Button::builder().label("Options").build();
    button_options.set_height_request(60);
    button_options.set_margin_bottom(50);
    left_grid.attach(&button_options, 1, 2, 1, 1);

    let button_import = Button::builder().label("Import").build();
    button_import.set_height_request(60);
    left_grid.attach(&button_import, 1, 3, 1, 1);

    let button_export = Button::builder().label("Export").build();
    button_export.set_height_request(60);
    left_grid.attach(&button_export, 1, 4, 1, 1);
}

/// fills the right main panel of the UI
fn fill_right_grid(right_grid: &Grid) {
    // action heading
    let actions_heading = Label::new(Some("Actions"));
    actions_heading.set_widget_name("heading");
    actions_heading.set_height_request(100);
    actions_heading.set_halign(gtk::Align::Start);
    right_grid.attach(&actions_heading, 1, 1, 1, 1);

    // grid for action buttons
    let action_grid = Grid::new();
    action_grid.set_widget_name("action_grid");
    action_grid.set_column_homogeneous(false);
    action_grid.set_row_homogeneous(true);
    right_grid.attach(&action_grid, 1, 2, 1, 1);

    let button_scan = Button::builder().label("Scan").build();
    action_grid.attach(&button_scan, 1, 2, 1, 1);

    let button_stress = Button::builder().label("Stress").build();
    action_grid.attach(&button_stress, 2, 2, 1, 1);

    // --- sensor heading
    let sensor_heading = Label::new(Some("Sensors"));
    sensor_heading.set_height_request(100);
    sensor_heading.set_widget_name("heading");
    sensor_heading.set_halign(gtk::Align::Start);
    right_grid.attach(&sensor_heading, 1, 3, 1, 1);

    // --- sensor grid
    let sensor_grid = Grid::new();
    sensor_grid.set_widget_name("sensor_grid");
    sensor_grid.set_column_homogeneous(false);
    //sensor_grid.set_row_homogeneous(true);
    right_grid.attach(&sensor_grid, 1, 4, 1, 1);

    // --- sensor data
    fill_sensor_grid(sensor_grid);

    // --- curve heading
    let curve_heading = Label::new(Some("Fan Curves"));
    curve_heading.set_height_request(100);
    curve_heading.set_widget_name("heading");
    curve_heading.set_halign(gtk::Align::Start);
    right_grid.attach(&curve_heading, 1, 5, 1, 1);

    // --- curve grid
    let curve_grid = Grid::new();
    curve_grid.set_widget_name("sensor_grid");
    right_grid.attach(&curve_grid, 1, 6, 1, 1);

    // --- curve data
    fill_curve_grid(curve_grid);
}

pub fn build_ui(app: &Application) {
    let main_window = ApplicationWindow::builder()
        .application(app)
        .title("BreezeBuddy")
        .build();

    // build main grid
    let main_grid = Grid::new();
    main_grid.set_widget_name("main_grid");
    main_grid.set_column_homogeneous(false);
    main_grid.set_row_homogeneous(true);
    main_window.set_child(Some(&main_grid));

    // left Grid
    let left_grid = Grid::new();
    left_grid.set_widget_name("left_grid");
    left_grid.set_column_homogeneous(true);
    left_grid.set_row_homogeneous(false);
    left_grid.set_width_request(300);
    fill_left_grid(&left_grid);
    main_grid.attach(&left_grid, 1, 1, 1, 1);

    // right Grid
    let right_grid = Grid::new();
    right_grid.set_widget_name("right_grid");
    right_grid.set_column_homogeneous(false);
    right_grid.set_row_homogeneous(false);
    fill_right_grid(&right_grid);
    main_grid.attach(&right_grid, 2, 1, 1, 1);

    main_window.present();
}
