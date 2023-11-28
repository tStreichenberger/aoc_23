extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
/// I didn't want to have to add each new day to the match arm so this does it automatically
/// Reading from a dir to generate the source is kinda hacky and probably the opposite of idiomatic
/// but I kinda just wanted to write a macro so don't @ me
pub fn get_day(day_num: TokenStream) -> TokenStream {
    let day_num = parse_macro_input!(day_num as syn::Ident);
    let dir_of_days = std::fs::read_dir("src/days").unwrap();
    let day_nums: Vec<_> = dir_of_days
        .into_iter()
        .filter_map(|dir_entry| {
            let dir_entry = dir_entry.ok()?;
            if dir_entry.file_type().ok()?.is_dir() {
                return None;
            }

            let file_name = dir_entry.file_name().to_string_lossy().to_string();
            let day_num: usize = file_name[3..5].parse().ok()?;
            Some(day_num)
        })
        .collect();
    let day_paths: Vec<TokenStream2> = day_nums
        .iter()
        .map(|day_num| {
            format!("crate::days::day{:02}::Day{:02}", day_num, day_num)
                .parse()
                .unwrap()
        })
        .collect();
    quote! {
        match #day_num {
            #(#day_nums => &#day_paths as &dyn Day,)*
             _ => panic!("No Solution found for day: {}", #day_num),
        }
    }
    .into()
}
