pub mod web_page {
  const INDEX: &str = include_str!("./index.html");
  const STYLES: &str = include_str!("./styles.css");
  const HEIC_SCRIPT: &str = include_str!("./heic2any.min.js");
  const FULLSCREEN_SCRIPT: &str = include_str!("./fullscreen.js");
  const SLIDES_SCRIPT: &str = include_str!("./slides.js");

  pub fn get_index() -> String {
    let mut scripts = String::from("");
    scripts.push_str(HEIC_SCRIPT);
    scripts.push_str(FULLSCREEN_SCRIPT);
    scripts.push_str(SLIDES_SCRIPT);
    
    INDEX.replace("{{styles}}", STYLES).replace("{{scripts}}", &scripts)
  }
}