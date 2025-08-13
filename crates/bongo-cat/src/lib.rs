use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro2::TokenStream;
use quote::quote;
use resvg::tiny_skia::Color;
use resvg::tiny_skia::Pixmap;
use resvg::usvg;
use resvg::usvg::Transform;
use syn::parse_macro_input;

const SVG_STR: &str = include_str!("../bongo-cat.svg");

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default)]
    binary: bool,
    #[darling(default)]
    width: Option<u32>,
    #[darling(default)]
    height: Option<u32>,
    #[darling(default)]
    left: bool,
    #[darling(default)]
    right: bool,
    #[darling(default)]
    both: bool,
}

/// #[bongo_cat(binary, width, height, left, right, both)]
/// struct BongoCat {}
#[proc_macro_attribute]
pub fn bongo_cat(
    attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mod_item = parse_macro_input!(item as syn::ItemMod);
    let mod_name = mod_item.ident;
    let mod_body = mod_item.content.unwrap().1;

    let raw = expand_with_attrs(attrs.into());
    quote! {
        mod #mod_name {
            #raw
            #(#mod_body)*
        }
    }
    .into()
}

fn expand_with_attrs(attrs: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(attrs.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };

    let args = match Args::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    expand_with_args(args)
}

fn expand_with_args(args: Args) -> TokenStream {
    let svg_opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(&SVG_STR, &svg_opt).unwrap();
    let image_size = tree.size();
    let mut transform = Transform::default();

    let width = if let Some(width) = args.width {
        transform = transform.post_scale(width as f32 / image_size.width(), 1.0);
        width
    } else {
        image_size.width().ceil() as u32
    };
    let height = if let Some(height) = args.height {
        transform = transform.post_scale(1.0, height as f32 / image_size.height());
        height
    } else {
        image_size.height().ceil() as u32
    };

    if args.binary {
        expand_binary(width, height, transform, args)
    } else {
        todo!();
        // expand_bmp(width, height, transform, args)
    }
}

fn expand_binary(width: u32, height: u32, transform: Transform, args: Args) -> TokenStream {
    let mut mod_body = TokenStream::new();
    let default_bitmap = read_binary(width, height, transform, false, false);
    mod_body.extend(quote! {
        pub const WIDTH: u32 = #width;
        pub const HEIGHT: u32 = #height;
        pub const DEFAULT: &[u8] = &[#(#default_bitmap),*];
    });

    if args.left {
        let left_up_bitmap = read_binary(width, height, transform, true, false);
        mod_body.extend(quote! {
            pub const LEFT_UP: &[u8] = &[#(#left_up_bitmap),*];
        });
    }
    if args.right {
        let right_up_bitmap = read_binary(width, height, transform, false, true);
        mod_body.extend(quote! {
            pub const RIGHT_UP: &[u8] = &[#(#right_up_bitmap),*];
        });
    }
    if args.both {
        let both_up_bitmap = read_binary(width, height, transform, true, true);
        mod_body.extend(quote! {
            pub const BOTH_UP: &[u8] = &[#(#both_up_bitmap),*];
        });
    }
    mod_body
}

fn read_binary(width: u32, height: u32, transform: Transform, left: bool, right: bool) -> Vec<u8> {
    let mut bitmap = vec![0u8; (width as usize + 8) / 8 * height as usize];
    let mut bit_iter = bitmap.iter_mut();
    let mut bit_byte = bit_iter.next().unwrap();
    let mut bit_index = 7;
    let mut pixmap = Pixmap::new(width, height).unwrap();
    pixmap.fill(Color::WHITE);
    let tree = read_tree(width, height, left, right);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    let threshold = 128u8;
    for y in 0..height {
        for x in 0..width {
            let pixel = pixmap.pixel(x, y).unwrap();
            let lum = (0.299 * pixel.red() as f32
                + 0.587 * pixel.green() as f32
                + 0.114 * pixel.blue() as f32) as u8;
            if lum < threshold {
                *bit_byte |= 1 << bit_index;
            }
            bit_index -= 1;
            if bit_index == -1 {
                bit_index = 7;
                bit_byte = bit_iter.next().unwrap();
            }
        }
        // row padding
        if bit_index != 7 && y != height - 1{
            bit_index = 7;
            bit_byte = bit_iter.next().unwrap();
        }
    }
    bitmap
}

#[allow(dead_code)]
fn expand_bmp(width: u32, height: u32, transform: Transform, _args: Args) -> TokenStream {
    let mut pixmap = Pixmap::new(width, height).unwrap();
    let tree = read_tree(width, height, true, true);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    let image = image::RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.take()).unwrap();
    image.save("bongo.bmp").unwrap();

    quote! {
        pub const WIDTH: u32 = #width;
        pub const HEIGHT: u32 = #height;
        pub const DEFAULT: [u8; 0] = [];
    }
}

fn read_tree(width: u32, height: u32, left: bool, right: bool) -> usvg::Tree {
    let mut svg_opt = usvg::Options::default();
    let mut css_vec = vec![];
    if width < 128 || height < 128 {
        css_vec.push(String::from(
            r#"
            .crab {
                display: none;
            }
        "#,
        ));
    }
    if left {
        css_vec.push(String::from(
            r#"
            #paw-left--down {
                display: none;
            }
        "#,
        ));
    } else {
        css_vec.push(String::from(
            r#"
            #paw-left--up {
                display: none;
            }
        "#,
        ));
    }
    if right {
        css_vec.push(String::from(
            r#"
            #paw-right--down {
                display: none;
            }
        "#,
        ));
    } else {
        css_vec.push(String::from(
            r#"
            #paw-right--up {
                display: none;
            }
        "#,
        ));
    }
    svg_opt.style_sheet = Some(css_vec.concat());
    usvg::Tree::from_str(&SVG_STR, &svg_opt).unwrap()
}
