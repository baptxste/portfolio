//! The views module contains the components for all Layouts and Routes for our app.

mod home;
pub use home::Home;

mod notes_home;
pub use notes_home::NotesHome;

mod note;
pub use note::NotePage;

mod tag;
pub use tag::TagPage;

mod navbar;
pub use navbar::Navbar;

mod cv; 
pub use cv::CvPage;

pub mod projects;

mod projects_home;
pub use projects_home::ProjectsHome;

mod project;
pub use project::ProjectPage;

mod not_found;
pub use not_found::NotFound;