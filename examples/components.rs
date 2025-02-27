// ANCHOR: all
use gtk::prelude::{
    ApplicationExt, ButtonExt, DialogExt, GtkWindowExt, ToggleButtonExt, WidgetExt,
};
use relm4::*;

// ANCHOR: header_model
struct HeaderModel;
// ANCHOR_END: header_model

// ANCHOR: header_msg
#[derive(Debug)]
enum HeaderOutput {
    View,
    Edit,
    Export,
}
// ANCHOR_END: header_msg

// ANCHOR: header

#[relm4::component]
impl SimpleComponent for HeaderModel {
    type Init = ();
    type Input = ();
    type Output = HeaderOutput;

    // ANCHOR: header_widgets
    view! {
        #[root]
        gtk::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &gtk::Box {
                add_css_class: "linked",
                #[name = "group"]
                gtk::ToggleButton {
                    set_label: "View",
                    set_active: true,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(HeaderOutput::View).unwrap()
                        }
                    },
                },
                gtk::ToggleButton {
                    set_label: "Edit",
                    set_group: Some(&group),
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(HeaderOutput::Edit).unwrap()
                        }
                    },
                },
                gtk::ToggleButton {
                    set_label: "Export",
                    set_group: Some(&group),
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(HeaderOutput::Export).unwrap()
                        }
                    },
                },
            }
        }
    }
    // ANCHOR_END: header_widgets

    fn init(
        _params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = HeaderModel;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

// ANCHOR_END: header

// ANCHOR: dialog_model
struct DialogModel {
    hidden: bool,
}
// ANCHOR_END: dialog_model

// ANCHOR: dialog_msg
#[derive(Debug)]
enum DialogInput {
    Show,
    Accept,
    Cancel,
}

#[derive(Debug)]
enum DialogOutput {
    Close,
}
// ANCHOR_END: dialog_msg

#[relm4::component]
impl SimpleComponent for DialogModel {
    type Init = bool;
    type Input = DialogInput;
    type Output = DialogOutput;

    // ANCHOR: dialog_widgets
    view! {
        gtk::MessageDialog {
            set_modal: true,
            #[watch]
            set_visible: !model.hidden,
            set_text: Some("Do you want to close before saving?"),
            set_secondary_text: Some("All unsaved changes will be lost"),
            add_button: ("Close", gtk::ResponseType::Accept),
            add_button: ("Cancel", gtk::ResponseType::Cancel),
            connect_response[sender] => move |_, resp| {
                sender.input(if resp == gtk::ResponseType::Accept {
                    DialogInput::Accept
                } else {
                    DialogInput::Cancel
                })
            }
        }
    }
    // ANCHOR_END: dialog_widgets

    fn init(
        params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = DialogModel { hidden: params };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    // ANCHOR: dialog_update
    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            DialogInput::Show => self.hidden = false,
            DialogInput::Accept => {
                self.hidden = true;
                sender.output(DialogOutput::Close).unwrap()
            }
            DialogInput::Cancel => self.hidden = true,
        }
    }
    // ANCHOR_END: dialog_update
}

// ANCHOR: app_model
#[derive(Debug)]
enum AppMode {
    View,
    Edit,
    Export,
}

#[derive(Debug)]
enum AppMsg {
    SetMode(AppMode),
    CloseRequest,
    Close,
}

struct AppModel {
    mode: AppMode,
    header: Controller<HeaderModel>,
    dialog: Controller<DialogModel>,
}
// ANCHOR_END: app_model

// ANCHOR: app
#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = AppMode;
    type Input = AppMsg;
    type Output = ();

    // ANCHOR: app_widgets
    view! {
        main_window = gtk::Window {
            set_default_width: 500,
            set_default_height: 250,
            set_titlebar: Some(model.header.widget()),

            gtk::Label {
                #[watch]
                set_label: &format!("Placeholder for {:?}", model.mode),
            },
            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::CloseRequest);
                gtk::Inhibit(true)
            }
        }
    }
    // ANCHOR_END: app_widgets

    // ANCHOR: app_init
    fn init(
        params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // ANCHOR: forward
        let header: Controller<HeaderModel> =
            HeaderModel::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    HeaderOutput::View => AppMsg::SetMode(AppMode::View),
                    HeaderOutput::Edit => AppMsg::SetMode(AppMode::Edit),
                    HeaderOutput::Export => AppMsg::SetMode(AppMode::Export),
                });
        // ANCHOR_END: forward

        let dialog = DialogModel::builder()
            .transient_for(root)
            .launch(true)
            .forward(sender.input_sender(), |msg| match msg {
                DialogOutput::Close => AppMsg::Close,
            });

        let model = AppModel {
            mode: params,
            header,
            dialog,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
    // ANCHOR_END: app_init

    // ANCHOR:app_update
    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::SetMode(mode) => {
                self.mode = mode;
            }
            AppMsg::CloseRequest => {
                self.dialog.sender().send(DialogInput::Show).unwrap();
            }
            AppMsg::Close => {
                relm4::main_application().quit();
            }
        }
    }
    // ANCHOR_END:app_update
}
// ANCHOR_END: app

fn main() {
    let relm = RelmApp::new("ewlm4.test.components");
    relm.run::<AppModel>(AppMode::Edit);
}
// ANCHOR_END: all
