// use codevis::Discard;
// crate::prodash::progress::Discard`,
use anyhow::Context;
use image::{ImageBuffer, Rgb};
use memmap2::MmapMut;
use prodash;
use prodash::progress::Discard;
use rfd::FileDialog;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use tempfile::Builder;

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
                edited(string) => { path = string; }
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
        in-out property <image> display_image: @image-url("assets/code.png");

        in-out property path_to_render <=> path_selecter.path;
        // in-out property <string> path_to_render: "";


        HorizontalBox {
            alignment: start;
            // Left settings panel
            VerticalBox {
                alignment: start;
                width: 300px;

                path_selecter := PathSelector {
                    select_path => {
                        root.path_to_render = root.select_render_path();
                    }
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
                source: root.display_image;
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

fn sage_image(
    img: ImageBuffer<Rgb<u8>, MmapMut>,
    img_path: &Path,
    mut progress: impl prodash::Progress,
) -> anyhow::Result<()> {
    let start = std::time::Instant::now();
    progress.init(
        Some(img.width() as usize * img.height() as usize * 3),
        Some(prodash::unit::dynamic_and_mode(
            prodash::unit::Bytes,
            prodash::unit::display::Mode::with_throughput(),
        )),
    );

    // There is no image format that can reasonably stream arbitrary image formats, so writing
    // isn't interactive.
    // I think the goal would be to write a TGA file (it can handle huge files in theory while being uncompressed)
    // and write directly into a memory map on disk, or any other format that can.
    // In the mean time, PNG files work as well even though some apps are buggy with these image resolutions.
    img.save(img_path)?;
    let bytes = img_path
        .metadata()
        .map_or(0, |md| md.len() as prodash::progress::Step);
    progress.inc_by(bytes);
    progress.show_throughput(start);
    Ok(())
}

fn main() -> anyhow::Result<()> {
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
        let main_window = main_window_weak.unwrap();

        let text: String = main_window.get_path_to_render().into();
        let path_to_render = Path::new(&text);

        let should_interrupt = AtomicBool::new(false);

        println!("path_to_render: {:?}", path_to_render);

        let (mut dir_contents, mut ignored) =
            codevis::unicode_content(&path_to_render, &[], Discard, &should_interrupt).unwrap();

        // Sort render order by path
        dir_contents
            .children_content
            .sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

        let ts = ThemeSet::load_defaults();
        let ss = SyntaxSet::load_defaults_newlines();
        let img = codevis::render(
            &dir_contents,
            Discard,
            &should_interrupt,
            &ss,
            &ts,
            codevis::render::Options::default(),
        )
        .unwrap();

        println!("rendered img!");

        let tmp_output_png = Builder::new()
            .prefix("temp-file")
            .suffix(".png")
            .rand_bytes(16)
            .tempfile()
            .unwrap();

        sage_image(img, tmp_output_png.path(), Discard).unwrap();
        let slint_img = slint::Image::load_from_path(tmp_output_png.path()).unwrap();
        main_window.set_display_image(slint_img);
        println!("set display image!");
    });

    main_window.run().unwrap();
    Ok(())
}
