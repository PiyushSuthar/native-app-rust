use druid::widget::{Flex, Label};
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder).title("Hello World");

    let data = 0_i8;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

fn ui_builder() -> impl Widget<i8> {
    Flex::column().with_child(Label::new("Hello World"))
}
