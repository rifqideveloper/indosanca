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