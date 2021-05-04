#![allow(deprecated)]

use gtk::prelude::*;

pub fn style(window: &gtk::ApplicationWindow) {
	let provider = gtk::CssProvider::new();

	let mut s = String::new();

	let mut add_color = |identifier: &str, color: &gdk::RGBA| {
		s.push_str("@define-color ");
		s.push_str(identifier);
		s.push_str(" ");
		s.push_str(&colorsys::Rgb::new(color.red * 255.0, color.green * 255.0, color.blue * 255.0, None).to_css_string());
		s.push_str(";\n");
	};

	let button = gtk::Button::new();
	let entry = gtk::Entry::new();
	add_color("c-neutral-100", &window.get_style_context().get_background_color(gtk::StateFlags::NORMAL));
	add_color("c-neutral-200", &entry.get_style_context().get_background_color(gtk::StateFlags::NORMAL));
	add_color("c-neutral-900", &button.get_style_context().get_color(gtk::StateFlags::NORMAL));

	let style = include_str!("../style/.build.css");
	s.push_str(style);

	provider.load_from_data(s.as_bytes()).expect("Failed to load CSS.");
	gtk::StyleContext::add_provider_for_screen(&gdk::Screen::get_default().expect("Error initializing GTK CSS provider."),
		&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}
