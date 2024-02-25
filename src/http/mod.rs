//This file mod.rs inside the folder exposes the folder as a module
pub mod request;
pub mod method;

//This exposes Request and Method into http module directly. Thus in main.rs we can write http::Request
pub use request::{ Request, ParseError };
pub use method::Method;
