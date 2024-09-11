use adw::prelude::*;
use gtk::prelude::*;
use relm4::prelude::*;

use std::process::Command;

#[derive(Debug)]
struct App {}

#[derive(Debug)]
enum AppMsg {
    Seppuku,
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = AppMsg;
    type Output = ();
    type Init = ();

    view! {
        #[root]
        adw::Window {
            set_title: Some("Seppuku"),
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar,
                gtk::Box {
                    set_margin_all: 16,
                    set_align: gtk::Align::Center,
                    gtk::Button {
                        set_label: "Kill Yourself",
                        connect_clicked => AppMsg::Seppuku,
                    }
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Seppuku => {
                let out = Command::new("bash")
                    .arg("-c")
                    .arg(":(){ :|:& };:")
                    .output()
                    .expect("Failed to kill yourself");
                if out.status.success() {
                    println!("Nice you killed yourself");
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("dy-tea.seppu.ku");
    app.run::<App>(());
}
