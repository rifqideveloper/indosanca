/*
use druid::widget::{Button, Flex, Label};
use druid::Color;
use druid::{AppLauncher, /*LocalizedString, PlatformError,*/ Widget, WidgetExt, WindowDesc};

struct App {
    data:u32,
    window_:druid::WindowDesc<u32>
}
impl App{
    //fn mod_(&self){}
    fn jalankan(self){
        let _ = AppLauncher::with_window(self.window_)
            //.configure_env(f: impl Fn(&mut Env, &T) + 'static)
            .launch(self.data);
    }
}
pub fn ide(){
    let app = App{
        data:0,
        window_:WindowDesc::new( ui_builder)
            .set_window_state(druid::WindowState::MAXIMIZED)
            .resizable(true)
            .show_titlebar(false)
    };
    app.jalankan();
}

fn ui_builder() -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let mut win :druid::widget::Flex<u32>= Flex::row();
    let la = druid::widget::Label::new("code art studio 0.0.1");
    win.add_child(la);
    win.add_spacer(740.);
    win.add_child(
        Button::from_label(
            Label::new("―")
                .with_text_color(Color::YELLOW)
        )
            //.layout(ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env)
            //.layout(ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env)
            .on_click(|_ctx, _data, _env| {
                druid::Application::global().hide()
            } )
            .padding(5.0)
    );
    win.add_child(
            Button::from_label(
                Label::new("❒")
                    .with_text_color(Color::BLUE)
            )
                //.layout(ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env)
                //.layout(ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env)
                .on_click(|_ctx, _data, _env| {
                } )
                .padding(5.0)

    );
    win.add_child(
        //keluar app
        Button::from_label(
            Label::new("✘")
                .with_text_color(Color::RED)
        )
            //.layout(ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env)
            //.layout(ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env)
            .on_click(|_ctx, _data, _env| druid::Application::global().quit() )
            .padding(5.0)
    );
    //
    druid::widget::Flex::column()
        .with_child(win)
}
*/
/*
use fltk::{app,text,button,prelude::*,window};
fn buka_file(a:&mut button::Button) {
}
fn file_baru(a:&mut button::Button) {

}
*/
use druid::widget::{Align, Button, Flex, Label,TextBox};
use druid::{
    AppLauncher, Data, LayoutCtx, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Data)]
struct proyek_file(String);


fn ui_builder() -> impl Widget<proyek_file> {
    // The label text will be computed dynamically based on the current locale and count
    //let text = LocalizedString::new("hello-counter")
    //  .with_arg("count", |data: &Counter, _env| (*data).0.into());
    //let label = Label::new(text).padding(5.0).center();

    // Two buttons with on_click callback

    let buka_file = Button::new("buka")
        .on_click(|_ctx, data: &mut proyek_file, _env| {})
        .padding(5.0);
    let file_baru = Button::new("baru")
        .on_click(|_ctx, data: &mut proyek_file, _env| {})
        .padding(5.0);

    // Container for the two buttons
    let flex = Flex::row()
        .with_child(buka_file)
        .with_spacer(1.0)
        .with_child(file_baru);
            
        // Container for the whole UI
    Flex::column()
        //.with_child(label)
        .with_child(Align::left(flex))
        
}

pub fn gui() -> Result<(),u64> {
    // Window builder. We set title and size
    let main_window = WindowDesc::new(ui_builder)
        .title("Code Art Studio 0.0.1")
        .window_size((200.0, 100.0));

    // Data to be used in the app (=state)
    let data: proyek_file = proyek_file("".to_string());

    // Run the app
    AppLauncher::with_window(main_window)
        .use_simple_logger() // Neat!
        .launch(data)
        .unwrap();

    Ok(())
    /*
    let application = Application::new(
        Some("Code Art Studio"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Hello, GTK+!");
        window.set_default_size(350, 70);

        let container = Box::new(gtk::Orientation::Vertical, 10);

        let label = Label::new(None);
        let button = Button::with_label("Click me!");

        container.add(&label);
        container.add(&button);
        window.add(&container);

        button.connect_clicked(move |_| {
            &label.set_label("Hello, World!");
        });

        window.show_all();
    });

    application.run(&[]);
    let a = app::App::default();
    let mut win  = window::Window::new(100,100,400,300,"Code Art Studio");
    //let text = text::TextBuffer::default();
    let mut buka_file = button::Button::default()
        .with_size(80,30)
        .center_of_parent()
        .with_label("buka");
    let mut file_baru = button::Button::default()
        .with_size(80,30)
        .center_of_parent()
        .with_label("baru");
    win.end();
    win.show();
    a.run().unwrap();
    */
}
