// use slint::Model;
use rfd::FileDialog;

slint::slint! {
    import { AboutSlint, Button, VerticalBox } from "std-widgets.slint";

    import { Button, GroupBox, SpinBox, ComboBox, CheckBox, LineEdit, TabWidget, VerticalBox, HorizontalBox,
        Slider, ProgressIndicator, SpinBox, Switch, Spinner, GridBox, StandardButton } from "std-widgets.slint";
    // import { GallerySettings } from "../gallery_settings.slint";
    // import { Page } from "page.slint";
    // export component Demo {


    // }

    component PathSelector {
        callback select_path;
        in-out property <string> path;
        HorizontalBox {
            LineEdit {
                placeholder-text: @tr("Path to render");
                text: path;
            }
            Button {
                text: "Browse";
                clicked => { root.select_path() }
            }
        }
    }

    export component MainWindow inherits Window {
        width: 1280px;
        height: 720px;
        callback select_render_path() -> string;

        HorizontalBox {
            alignment: start;
            // Left settings panel
            VerticalBox {
                alignment: start;
                width: 300px;

                PathSelector {
                    select_path => { self.path = root.select_render_path() }
                }

                // HorizontalBox {
                //     alignment: start;
                //     StandardButton { kind: ok; }
                //     StandardButton { kind: apply; }
                // }
                // StandardButton { kind: cancel; }
                Switch {
                    text: @tr("Readable");
                    checked: true;
                }

                HorizontalBox {
                    Text {
                        vertical-alignment: center;
                        text: @tr("Theme: ");
                    }
                    ComboBox {
                        model: ["Solarized (dark)", "Solarized (light)", "InspiredGitHub", "base16-eighties.dark", "base16-mocha.dark", "base16-ocean.dark", "base16-ocean.light"];
                    }
                }
            }

            // image
            Image {
                source: @image-url("assets/code.png");
            }
            VerticalBox {
                // Image of output

            }

        }
            // VerticalBox {
            // alignment: start;
            // //Text {
            // //    text: "Hello World!";
            // //    font-size: 24px;
            // //    horizontal-alignment: center;
            // //}
            // // AboutSlint {
            // //     preferred-height: 150px;
            // // }

            // HorizontalBox {
            //     StandardButton { kind: ok; }
            //     StandardButton { kind: apply; }
            //     StandardButton { kind: cancel; }
            // }

            // HorizontalLayout { alignment: center; Button { text: "OK!"; } }

    }
}

fn main() {
    let main_window = MainWindow::new().unwrap();

    let main_window_weak = main_window.as_weak();
    main_window.on_select_render_path(move || {
        let folder = FileDialog::new().pick_folder();

        println!("{:?}", folder);

        match folder {
            Some(folder) => folder.to_string_lossy().to_string().into(),
            None => "".into(),
        }
    });

    main_window.run().unwrap();
}
