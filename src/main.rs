// use codevis::Discard;
// crate::prodash::progress::Discard`,
use codevis::render::BgColor;
use image::{ImageBuffer, Rgb};
use memmap2::MmapMut;
use prodash;
use prodash::progress::Discard;
use rfd::FileDialog;
use std::ffi::OsString;
use std::path::Path;
use std::sync::atomic::AtomicBool;
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
        callback select_render_path();
        callback render();
        in-out property <image> display_image: @image-url("assets/code.png");

        in-out property path_to_render <=> path_selecter.path;
        out property <bool> readable <=> readable_switch.checked;
        out property <string> theme <=> theme_combobox.current-value;
        out property <int> bg_pixel_color <=> bg_pixel_color_combobox.current-index;
        out property <int> tab_spaces <=> tab_spaces_spinbox.value;
        out property <bool> line_nums <=> line_num_switch.checked;
        out property <bool> show_filenames <=> file_names_switch.checked;
        out property <int> aspect_x <=> aspect_x_spinbox.value;
        out property <int> aspect_y <=> aspect_y_spinbox.value;
        out property <bool> force_full_columns <=> force_full_columns_switch.checked;
        out property <string> ignored_extensions <=> ignored_extension_lineedit.text;

        HorizontalBox {
            alignment: start;
            // Left settings panel
                VerticalBox {
                    alignment: center;
                    GroupBox {
                        title: @tr("Settings");
                    VerticalBox {
                        alignment: start;
                        width: 300px;

                        path_selecter := PathSelector {
                            select_path => {
                                root.select_render_path();
                            }
                        }

                        readable_switch := Switch {
                            text: @tr("Readable");
                            checked: false;
                        }

                        HorizontalBox {
                            Text {
                                vertical-alignment: center;
                                text: @tr("Theme: ");
                            }
                            theme_combobox := ComboBox {
                                model: ["Solarized (dark)", "Solarized (light)", "InspiredGitHub", "base16-eighties.dark", "base16-mocha.dark", "base16-ocean.dark", "base16-ocean.light"];
                            }
                        }

                        HorizontalBox {
                            Text {
                                vertical-alignment: center;
                                text: @tr("bg-pixel-color: ");
                            }
                            bg_pixel_color_combobox := ComboBox {
                                model: ["style", "style-checkerboard-darken", "style-checkerboard-brighten", "helix-editor"];
                            }
                        }

                        //tab spaces
                        HorizontalBox {
                            Text {
                                vertical-alignment: center;
                                text: @tr("tab spaces: ");
                            }
                            tab_spaces_spinbox := SpinBox {
                                value: 4;
                                minimum: 1;
                                maximum: 16;
                            }
                        }

                        line_num_switch := Switch {
                            text: @tr("line numbers");
                            checked: false;
                        }

                        file_names_switch := Switch {
                            text: @tr("file names");
                            checked: false;
                        }

                        HorizontalBox {
                            Text {
                                vertical-alignment: center;
                                text: @tr("Aspect Ratio: ");
                            }
                            aspect_x_spinbox := SpinBox {
                                value: 16;
                                minimum: 1;
                                maximum: 2147483647; // Maximum for i32
                            }
                            aspect_y_spinbox := SpinBox {
                                value: 9;
                                minimum: 1;
                                maximum: 2147483647; // Maximum for i32
                            }
                        }

                        // force_full_columns
                        force_full_columns_switch := Switch {
                            text: @tr("force full columns");
                            checked: true;
                        }

                        // ignored extensions
                        VerticalBox {
                            Text {
                                vertical-alignment: center;
                                text: @tr("ignored extensions (space separated):");
                            }
                            ignored_extension_lineedit := LineEdit {
                                placeholder-text: @tr("ignored extensions");
                            }
                        }


                        Button {
                            primary: true;
                            text: "Render";
                            clicked => { root.render() }
                        }
                    }
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
        let main_window = main_window_weak.unwrap();

        let folder = FileDialog::new().pick_folder();

        println!("{:?}", folder);

        if let Some(folder) = folder {
            main_window.set_path_to_render(folder.to_string_lossy().to_string().into());
        }
    });

    let main_window_weak = main_window.as_weak();
    main_window.on_render(move || {
        let main_window = main_window_weak.unwrap();

        let text: String = main_window.get_path_to_render().into();
        let path_to_render = Path::new(&text);

        let should_interrupt = AtomicBool::new(false);

        println!("path_to_render: {:?}", path_to_render);

        let ignored_extensions: Vec<OsString> = main_window
            .get_ignored_extensions()
            .split_whitespace()
            .map(|ext| OsString::from(ext))
            .collect();
        let (mut dir_contents, mut _ignored) = codevis::unicode_content(
            &path_to_render,
            &ignored_extensions,
            Discard,
            &should_interrupt,
        )
        .unwrap();

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
            // codevis::render::Options::default(),
            {
                let theme: String = main_window.get_theme().into();
                codevis::render::Options {
                    // Set specific fields here
                    readable: main_window.get_readable(),
                    theme: &theme.clone(),
                    bg_color: {
                        match main_window.get_bg_pixel_color() {
                            0 => BgColor::Style,
                            1 => BgColor::StyleCheckerboardDarken,
                            2 => BgColor::StyleCheckerboardBrighten,
                            3 => BgColor::HelixEditor,
                            _ => BgColor::Style,
                        }
                    },
                    tab_spaces: main_window.get_tab_spaces().try_into().unwrap(),
                    line_nums: main_window.get_line_nums(),
                    show_filenames: main_window.get_show_filenames(),
                    target_aspect_ratio: {
                        let aspect_x = main_window.get_aspect_x();
                        let aspect_y = main_window.get_aspect_y();
                        aspect_x as f64 / aspect_y as f64
                    },
                    force_full_columns: main_window.get_force_full_columns(),

                    // Set the rest of the fields to their default values
                    ..Default::default()
                }
            },
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
