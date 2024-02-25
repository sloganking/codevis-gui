// use codevis::Discard;
// crate::prodash::progress::Discard`,
use codevis::render::BgColor;
use image::{ImageBuffer, Rgb};
use memmap2::MmapMut;
use prodash;
use prodash::progress::Discard;
use rfd::FileDialog;
use slint::{Rgb8Pixel, SharedPixelBuffer};
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
        callback edited;
        in-out property <string> path <=> path_selector_lineedit.text;
        HorizontalBox {
            path_selector_lineedit := LineEdit {
                placeholder-text: @tr("Path to render");
                edited(string) => {
                    root.edited();
                }
            }
            path_selector_button := Button {
                text: "Browse";
                clicked => {
                    root.select_path();
                    root.edited();
                }
            }
        }
    }

    export component MainWindow inherits Window {
        title: "codevis-gui";
        width: 1280px;
        height: 720px;
        callback select_render_path();
        callback render();
        in property <image> display_image: @image-url("");
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
        out property <bool> auto_rendering <=> auto_render_switch.checked;
        out property <int> auto_render_limit <=> auto_render_spinbox.value;
        out property <int> column_width <=> column_width_spinbox.value;

        HorizontalBox {
            alignment: stretch;
            // Left settings panel

            HorizontalLayout {
                alignment: start;
                VerticalBox {
                    alignment: center;
                    GroupBox {
                        title: @tr("Settings");
                        VerticalBox {
                            alignment: start;

                            path_selecter := PathSelector {
                                select_path => {
                                    root.select_render_path();
                                }
                                edited => {
                                    if auto_render_switch.checked {
                                        root.render()
                                    }
                                }
                            }

                            readable_switch := Switch {
                                text: @tr("Readable");
                                checked: false;
                                toggled => {
                                    if auto_render_switch.checked {
                                        root.render()
                                    }
                                }
                            }

                            HorizontalBox {
                                Text {
                                    vertical-alignment: center;
                                    text: @tr("Theme: ");
                                }
                                theme_combobox := ComboBox {
                                    model: ["Solarized (dark)", "Solarized (light)", "InspiredGitHub", "base16-eighties.dark", "base16-mocha.dark", "base16-ocean.dark", "base16-ocean.light"];
                                    selected => {
                                        if auto_render_switch.checked {
                                            root.render()
                                        }
                                    }
                                }
                            }

                            HorizontalBox {
                                Text {
                                    vertical-alignment: center;
                                    text: @tr("bg-pixel-color: ");
                                }
                                bg_pixel_color_combobox := ComboBox {
                                    model: ["style", "style-checkerboard-darken", "style-checkerboard-brighten", "helix-editor"];
                                    selected => {
                                        if auto_render_switch.checked {
                                            root.render()
                                        }
                                    }
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
                                    edited => {
                                        if auto_render_switch.checked {
                                            root.render()
                                        }
                                    }
                                }
                            }

                            line_num_switch := Switch {
                                text: @tr("line numbers");
                                checked: false;
                                toggled => {
                                    if auto_render_switch.checked {
                                        root.render()
                                    }
                                }
                            }

                            file_names_switch := Switch {
                                text: @tr("file names");
                                checked: false;
                                toggled => {
                                    if auto_render_switch.checked {
                                        root.render()
                                    }
                                }
                            }

                            Text {
                                // vertical-alignment: center;
                                text: @tr("Aspect Ratio:");
                            }
                            HorizontalBox {
                                aspect_x_spinbox := SpinBox {
                                    value: 16;
                                    minimum: 1;
                                    maximum: 2147483647; // Maximum for i32
                                    edited => {
                                        if auto_render_switch.checked {
                                            root.render()
                                        }
                                    }
                                }
                                Text {
                                    vertical-alignment: center;
                                    text: @tr("by");
                                }
                                aspect_y_spinbox := SpinBox {
                                    value: 9;
                                    minimum: 1;
                                    maximum: 2147483647; // Maximum for i32
                                    edited => {
                                        if auto_render_switch.checked {
                                            root.render()
                                        }
                                    }
                                }
                            }

                            // force_full_columns
                            force_full_columns_switch := Switch {
                                text: @tr("force full columns");
                                checked: true;
                                toggled => {
                                    if auto_render_switch.checked {
                                        root.render()
                                    }
                                }
                            }

                            // column_width
                            HorizontalLayout {
                                Text {
                                    vertical-alignment: center;
                                    text: @tr("column width: ");
                                }
                                column_width_spinbox := SpinBox {
                                    value: 100;
                                    minimum: 1;
                                    maximum: 400;
                                    edited => {
                                        if auto_render_switch.checked {
                                            root.render()
                                        }
                                    }
                                }
                            }

                            // ignored extensions
                            VerticalBox {
                                Text {
                                    vertical-alignment: center;
                                    text: @tr("ignored extensions (space separated):");
                                }
                                ignored_extension_lineedit := LineEdit {
                                    placeholder-text: @tr("ignored extensions");
                                    edited => {
                                        if auto_render_switch.checked {
                                            root.render()
                                        }
                                    }
                                }
                            }

                            // auto render
                            auto_render_switch := Switch {
                                text: @tr("Auto render");
                                checked: false;
                                toggled => {
                                    if auto_render_switch.checked {
                                        root.render()
                                    }
                                }
                            }
                            HorizontalLayout {
                                Text {
                                    vertical-alignment: center;
                                    text: @tr("Auto render line count limit: ");
                                }

                                auto_render_spinbox := SpinBox {
                                    value: 100000;
                                    minimum: 1;
                                    maximum: 2147483647; // Maximum for i32
                                    enabled: auto_render_switch.checked;
                                }
                            }



                            Button {
                                primary: true;
                                text: "Render";
                                clicked => { root.render() }
                                enabled: !auto_render_switch.checked;
                            }
                        }
                    }
                }
            }

            Image {
                horizontal-stretch: 1;
                source: root.display_image;
                image-rendering: pixelated;
            }
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

        if !path_to_render.is_dir() {
            let pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::new(1, 1);
            let slint_img = slint::Image::from_rgb8(pixel_buffer);
            main_window.set_display_image(slint_img);
            return;
        }

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

        if main_window.get_auto_rendering() {
            // count lines in dir_contents
            let mut total_line_count = 0;
            for (_, text) in &dir_contents.children_content {
                if !text.is_empty() {
                    // println!("text: {}\n\n", text);
                    total_line_count += text.lines().count();
                }
            }

            println!("total_line_count: {}", total_line_count);

            // don't render if too much or too little content
            if total_line_count == 0
                || total_line_count > main_window.get_auto_render_limit().try_into().unwrap()
            {
                let pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::new(1, 1);
                let slint_img = slint::Image::from_rgb8(pixel_buffer);
                main_window.set_display_image(slint_img);
                return;
            }
        }

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
                    column_width: main_window.get_column_width().try_into().unwrap(),

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
