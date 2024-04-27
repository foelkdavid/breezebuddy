use gtk4 as gtk;
use gtk::{prelude::*, Application, ApplicationWindow, Button, Grid, Label};


fn fill_left_grid(left_grid: &Grid){
    let main_heading = Label::new(Some("Breeze Buddy"));
    main_heading.set_widget_name("heading");
    main_heading.set_margin_top(20);
    main_heading.set_margin_bottom(100);
    left_grid.attach(&main_heading, 1, 1, 1, 1);

    let button_options = Button::builder()
        .label("Options")
        .build();
    button_options.set_height_request(60);
    button_options.set_margin_bottom(50);
    left_grid.attach(&button_options, 1, 2, 1, 1);

    let button_import = Button::builder()
        .label("Import")
        .build();
    button_import.set_height_request(60);
    left_grid.attach(&button_import, 1, 3, 1, 1);

    let button_export = Button::builder()
        .label("Export")
        .build();
    button_export.set_height_request(60);
    left_grid.attach(&button_export, 1, 4, 1, 1);
}


fn fill_right_grid(right_grid: &Grid){

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

    let button_scan = Button::builder()
        .label("Scan")
        .build();
    action_grid.attach(&button_scan, 1, 2, 1, 1);

    let button_stress = Button::builder()
        .label("Stress")
        .build();
    action_grid.attach(&button_stress, 2, 2, 1, 1);

    // curve heading
    let curve_heading = Label::new(Some("Fan Curves"));
    curve_heading.set_height_request(100);
    curve_heading.set_widget_name("heading");
    curve_heading.set_halign(gtk::Align::Start);
    right_grid.attach(&curve_heading, 1, 3, 1, 1);

    // curves - these need to be created dynamically
    let curve1 = Label::new(Some("Curve 1"));
    curve1.set_widget_name("curve");
    curve1.set_height_request(300);
    right_grid.attach(&curve1, 1, 4, 1, 1);

    let curve2 = Label::new(Some("Curve 2"));
    curve2.set_widget_name("curve");
    curve2.set_height_request(300);
    right_grid.attach(&curve2, 2, 4, 1, 1);

    let curve3 = Label::new(Some("Curve 3"));
    curve3.set_widget_name("curve");
    curve3.set_height_request(300);
    right_grid.attach(&curve3, 1, 5, 1, 1);

    let curve4 = Label::new(Some("Curve 4"));
    curve4.set_widget_name("curve");
    curve4.set_height_request(300);
    right_grid.attach(&curve4, 2, 5, 1, 1);



}


pub fn build_ui(app: &Application){
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
    right_grid.set_column_homogeneous(true);
    right_grid.set_row_homogeneous(false);
    right_grid.set_width_request(1200);
    fill_right_grid(&right_grid);
    main_grid.attach(&right_grid, 2, 1, 1, 1);

    main_window.present();

}