use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use regex::Regex;
use std::rc::Rc;

#[no_mangle]
pub fn _start() {
  proxy_wasm::set_log_level(LogLevel::Trace);
  proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { 
     Box::new(DemoPlugin {
        expresion: Rc::<String>::new("".to_string()),
        header_content: String::new(),
     }) 
  });
}

struct DemoPlugin {
  header_content: String,
  expresion: Rc<String>,
}

impl RootContext for DemoPlugin {

  fn on_configure(&mut self, _: usize) -> bool {
    if let Some(config_bytes) = self.get_plugin_configuration() {
      self.expresion = Rc::new(String::from_utf8(config_bytes).unwrap())
    }
    true
  }

  fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
     Some(Box::new(DemoPlugin {
       expresion: self.expresion.clone(),
       header_content: String::new(),
     }))
  }

  fn get_type(&self) -> Option<ContextType> {
    Some(ContextType::HttpContext)
  }

}


impl HttpContext for DemoPlugin {

    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
      if let Some(origin) = self.get_http_request_header("origin") {
           self.header_content  = origin.clone(); 
      } else { 
           self.header_content = "".to_string();
      }
      Action::Continue
    }
    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
      let re = Regex::new(r"^https?.+(\.demo\.com)$").unwrap();
      if self.header_content == ""  {
        self.set_http_response_header("Access-Control-Allow-Origin", Some(""));
        Action::Continue
      } else {
        if re.is_match(self.header_content.as_str()) {
             self.set_http_response_header("Access-Control-Allow-Origin", Some(self.header_content.as_str()));
             self.set_http_response_header("Vary", Some("Origin"));   
             self.set_http_response_header("Access-Control-Allow-Credentials", Some("true"));
             Action::Continue
        } else {
             println!("HEA - Access forbidden.");
             self.set_http_response_header("Access-Control-Allow-Origin", Some(""));
             Action::Continue
        }  
      }
   }  
}

impl Context for DemoPlugin {}
