extern crate gdk;
extern crate gdk_sys as gdk_ffi;
extern crate gio_sys as gio_ffi;
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk;
extern crate gtk_sys as gtk_ffi;
extern crate libc;
#[macro_use(nautilus_module, nautilus_menu_item_activate_cb)]
extern crate nautilus_extension;
extern crate url;

pub mod column_provider;
pub mod info_provider;
pub mod menu_provider;
pub mod property_page_provider;
pub mod tags_list;
mod tmsu_commands;
pub mod tmsu_nautilus;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
