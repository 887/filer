//extern crate webkit2gtk;

//#[cfg(feature = "v2_6")]
//use webkit2gtk::UserContentManager;

//use webkit2gtk::{
    //SettingsExt,
    //WebContext,
    //WebContextExt,
    //WebView,
    //WebViewExt,
    //WebViewExtManual
//};

//let context = WebContext::get_default().unwrap();
//#[cfg(feature = "v2_4")]
//context.set_web_extensions_initialization_user_data(&"webkit".to_variant());
//#[cfg(feature = "v2_6")]
//let webview = WebView::new_with_context_and_user_content_manager(&context, &UserContentManager::new());
//#[cfg(not(feature = "v2_6"))]
//let webview = WebView::new_with_context(&context);

//// webview.load_uri("https://crates.io/");

////for rendering pdfs we need poppler, the rest could be done with a web and/or
////textview/codeview/imageview

////https://wiki.gnome.org/Projects/Vala/PopplerSample
//webview.load_uri("file:///media/c/books - pdf/ViE-Requiem.pdf");

//let settings = WebViewExt::get_settings(&webview).unwrap();
//settings.set_enable_developer_extras(true);

//main_box.add(&webview);
