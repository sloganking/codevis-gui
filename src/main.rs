// use codevis::Discard;
// crate::prodash::progress::Discard`,
use prodash;
use prodash::progress::Discard;
use rfd::FileDialog;
use std::path::Path;
use std::sync::atomic::AtomicBool;

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
        callback render();

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
                Button {
                    primary: true;
                    text: "Render";
                    clicked => { root.render() }
                }
            }

            Image {
                source: @image-url("assets/code.png");
            }

            // Rectangle{
            //     background: red;
            //     // alignment: center;
            //     // horizontal-stretch: 1000%;
            //     // width:  200px;
            //     preferred-width: 100%;
            //     Spinner {
            //         vertical-stretch: 1;
            //         // width: 100px;
            //         min-width: 100px;
            //         min-height: 100px;
            //         // progress: i-progress-indicator.progress;
            //         indeterminate: true;
            //     }
            // }


        }
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

    main_window.on_render(move || {
        println!("Render!");

        let path = Path::new("./");

        let (mut dir_contents, mut ignored) =
            codevis::unicode_content(&path, &[], Discard, &AtomicBool::new(false)).unwrap();

        // codevis::render(&main_window_weak);
    });

    main_window.run().unwrap();
}
