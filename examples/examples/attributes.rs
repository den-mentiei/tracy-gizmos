#![allow(dead_code)]

#[tracy_gizmos::capture]
fn main() {
	work();
}

#[tracy_gizmos::instrument("Main")]
fn work() {
	println!("I am instrumented.");
	std::thread::sleep(std::time::Duration::from_millis(100));
}

#[tracy_gizmos::instrument]
fn r#fn() {
}

#[tracy_gizmos::instrument]
unsafe fn i_am_unsafe() {
}

#[tracy_gizmos::instrument]
pub extern "C" fn i_am_extern_c() -> i32 { 0 }

#[tracy_gizmos::instrument]
fn i_have_generics<'a, T: std::fmt::Debug, K>() where K: Eq + Ord {}

// The following ones can't be instrumented:

// #[tracy_gizmos::instrument]
// struct NotAllowed {}

// #[tracy_gizmos::instrument]
// const fn i_am_const() {
// }

// #[tracy_gizmos::instrument]
// async fn i_am_async() {
// }

// #[tracy_gizmos::instrument]
// const unsafe fn i_am_const_unsafe() {
// }

// #[tracy_gizmos::instrument]
// async unsafe fn i_am_unsafe_async() {
// }

// extern "stdcall" {
// 	#[tracy_gizmos::instrument]
// 	fn i_have_no_body(x: i32) -> i32;
// }
