use {
    adw::subclass::application_window::AdwApplicationWindowImpl,
    gtk::{
        CompositeTemplate,
        glib::{
            self,
            subclass::{InitializingObject, types::ObjectSubclass},
        },
        subclass::prelude::*,
    },
};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/GtkDemoApp/window.ui")]
pub struct Window {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "GtkDemoApp";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl Window {
    // #[template_callback]
    // fn handle_button_clicked(&self, button: &CustomButton) {
    //     let number_increased = self.number.get() + 1;
    //     self.number.set(number_increased);
    //     button.set_label(&number_increased.to_string())
    // }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
