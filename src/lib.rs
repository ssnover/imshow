use iced::{
    keyboard::{key::Named, Key},
    widget::{canvas, scrollable::Direction},
    Element, Task,
};
use image::{DynamicImage, RgbaImage};

pub fn imshow(img: &DynamicImage) {
    let img = img.to_rgba8();

    iced::application("imshow", handle_key, show_image)
        .centered()
        .subscription(|_| {
            iced::keyboard::on_key_release(|key, _| {
                if let Key::Named(Named::Enter) = key {
                    Some(())
                } else {
                    None
                }
            })
        })
        .exit_on_close_request(true)
        .run_with(|| (img, Task::none()))
        .unwrap()
}

fn handle_key(_: &mut RgbaImage, _msg: ()) -> Task<()> {
    iced::window::get_latest().and_then(|id| iced::window::close::<()>(id))
}

fn show_image(img: &RgbaImage) -> Element<()> {
    iced::widget::Responsive::new(|size| {
        iced::widget::scrollable(
            iced::widget::canvas(ImshowCanvas::new(img))
                .width(size.width)
                .height(size.height), // todo: this needs some kind of dynamic scaling to match aspect ratio of image
        )
        .direction(Direction::Both {
            vertical: Default::default(),
            horizontal: Default::default(),
        })
        .into()
    })
    .into()
}

struct ImshowCanvas<'a> {
    img: &'a RgbaImage,
}

impl<'a> ImshowCanvas<'a> {
    pub fn new(img: &'a RgbaImage) -> ImshowCanvas<'a> {
        Self { img }
    }
}

impl<'a, M> canvas::Program<M> for ImshowCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<iced::Renderer>> {
        let handle = iced::widget::image::Handle::from_rgba(
            self.img.width(),
            self.img.height(),
            self.img.clone().into_raw(),
        );
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        frame.draw_image(bounds, canvas::Image::new(handle));
        vec![frame.into_geometry()]
    }
}
